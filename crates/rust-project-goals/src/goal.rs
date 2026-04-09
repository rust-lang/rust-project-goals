use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::{collections::BTreeSet, path::PathBuf};

use spanned::{Error, Result, Spanned};

use crate::config::{Configuration, TeamAskDetails};
use crate::gh::issue_id::{IssueId, Repository};
use crate::gh::issues::{checkboxes, ExistingGithubIssue};
use crate::markwaydown::{self, Section, Table};
use crate::re::{self, CHAMPION_METADATA};
use crate::team::{self, TeamName};
use crate::util::{self, commas, markdown_files};
use rust_project_goals_json::{GithubIssueState, Progress};

/// Data parsed from a goal file in the expected format
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoalDocument {
    /// Path relative to the current directory (`book.toml`)
    pub path: PathBuf,

    /// Path relative to the directory of goals this goal is a part of,
    /// and hence suitable for links in other markdown files.
    pub link_path: Arc<PathBuf>,

    /// Metadata loaded from the header in the goal
    pub metadata: Metadata,

    /// Text from the summary section
    pub summary: String,

    /// The "plan" for completing the goal (includes things owners will do as well as team asks).
    /// Only present for old format goals (pre-2026).
    pub goal_plans: Vec<GoalPlan>,

    /// Owners of any task that are not team asks.
    /// For old format: extracted from "Ownership and team asks" plan items.
    /// For new format: extracted from `| Task | Owner(s) | Notes |` tables in the task tree.
    pub task_owners: BTreeSet<String>,

    /// How teams are involved with this goal - either through specific asks (old format)
    /// or support levels (new format).
    pub team_involvement: TeamInvolvement,

    /// Hierarchical task structure parsed from "Work items over the next year".
    /// When it has children, roadmap tables show one row per child instead of one row for the goal.
    pub task_tree: TaskTree,
}

/// Data parsed from a roadmap file (`roadmap-*.md`).
/// Roadmaps are narrative documents that group goals by theme.
/// They have simpler metadata than goals (no status, tracking issues, or team asks).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoadmapDocument {
    /// Path relative to the current directory (`book.toml`)
    pub path: PathBuf,

    /// Path relative to the directory of roadmaps this is a part of,
    /// and hence suitable for links in other markdown files.
    pub link_path: Arc<PathBuf>,

    /// The full title from the `#` heading
    pub title: Spanned<String>,

    /// Short title from metadata table (or title if absent)
    pub short_title: Spanned<String>,

    /// Text from the "What and why" metadata row
    pub what_and_why: String,

    /// Point of contact (e.g. `@username` or "TBD")
    pub point_of_contact: String,

    /// Text from the summary section
    pub summary: String,
}

impl RoadmapDocument {
    /// Load a roadmap document from a markdown file.
    /// Returns None if the file doesn't have roadmap metadata.
    fn load(path: &Path, link_path: &Path) -> Result<Option<Self>> {
        let sections = markwaydown::parse(path)?;

        let Some(RoadmapMetadata {
            title,
            short_title,
            what_and_why,
            point_of_contact,
        }) = extract_roadmap_metadata(&sections)?
        else {
            return Ok(None);
        };

        let summary = extract_summary(&sections)?.unwrap_or_else(|| title.content.clone());

        Ok(Some(RoadmapDocument {
            path: path.to_path_buf(),
            link_path: Arc::new(link_path.to_path_buf()),
            title,
            short_title,
            what_and_why,
            point_of_contact,
            summary,
        }))
    }
}

/// Metadata loaded from the goal header
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    pub title: Spanned<String>,
    pub short_title: Spanned<String>,
    pub pocs: String,
    pub status: Spanned<Status>,
    pub tracking_issue: Option<IssueId>,
    pub table: Spanned<Table>,

    /// For each table entry like `[T-lang] champion`, we create an entry in this map
    pub champions: BTreeMap<&'static TeamName, Spanned<String>>,

    /// Roadmap themes this goal belongs to (zero or more)
    pub roadmap: Themes,

    /// Highlight themes this goal belongs to (zero or more)
    pub highlight: Themes,

    /// Needs of this goal (zero or more), e.g. "Funding", "Contributor"
    pub needs: Themes,

    /// Optional "What and why" description from metadata table
    pub what_and_why: Option<String>,

    /// Optional "Timespan" override (e.g. "2026–2027"). Defaults to the milestone directory name.
    pub timespan: Option<String>,
}

impl Metadata {
    /// True if this goal needs a contributor (i.e. is "help wanted").
    pub fn is_help_wanted(&self) -> bool {
        self.needs.contains("Contributor")
    }
}

/// A set of theme names parsed from metadata rows.
/// Used for both roadmap and highlight themes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Themes {
    themes: Vec<Spanned<String>>,
}

impl Themes {
    /// Returns true if no themes are present.
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    /// Returns true if any theme is present.
    pub fn is_some(&self) -> bool {
        !self.themes.is_empty()
    }

    /// Returns true if the set contains a theme matching the given value (trimmed comparison).
    pub fn contains(&self, value: &str) -> bool {
        self.themes.iter().any(|t| t.content.trim() == value.trim())
    }

    /// Iterate over all theme names as &str.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.themes.iter().map(|t| t.content.as_str())
    }

    /// Returns the first theme name, if any.
    pub fn first(&self) -> Option<&str> {
        self.themes.first().map(|t| t.content.as_str())
    }

    /// Iterate over all theme entries with their spans.
    pub fn iter_spanned(&self) -> impl Iterator<Item = &Spanned<String>> {
        self.themes.iter()
    }

    fn push(&mut self, theme: Spanned<String>) {
        self.themes.push(theme);
    }
}

pub const TRACKING_ISSUE_ROW: &str = "Tracking issue";

/// A single row from a `| Task | Owner(s) | Notes |` table
/// under "Work items over the next year" (2026+ format).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskItem {
    pub task: Spanned<String>,
    pub owners: String,
    pub notes: String,
}

/// Hierarchical task structure parsed from "Work items over the next year" (2026+ format).
/// The root node represents the goal itself; children represent `####` subsections.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskTree {
    /// Title of this node (goal title for root, heading text for children)
    pub title: Spanned<String>,

    /// Task items from the `| Task | Owner(s) | Notes |` table in this section
    pub tasks: Vec<TaskItem>,

    /// Effective roadmap themes (parent UNION own for children; from metadata for root)
    pub roadmap: Themes,

    /// Effective timespan (own if present, else parent's)
    pub timespan: Option<String>,

    /// Effective "what and why" (own if present, else parent's)
    pub what_and_why: Option<String>,

    /// Child nodes from `####` subsections
    pub children: Vec<TaskTree>,
}

impl TaskTree {
    /// Build a root task tree node from goal metadata.
    fn root(metadata: &Metadata, tasks: Vec<TaskItem>, children: Vec<TaskTree>) -> Self {
        TaskTree {
            title: metadata.title.clone(),
            tasks,
            roadmap: metadata.roadmap.clone(),
            timespan: metadata.timespan.clone(),
            what_and_why: metadata.what_and_why.clone(),
            children,
        }
    }

    /// Recursively collect all owner usernames from task items across the tree.
    pub fn all_task_owners(&self) -> BTreeSet<String> {
        let mut owners = BTreeSet::new();
        for task in &self.tasks {
            for username in owner_usernames(&task.owners) {
                owners.insert(username.to_string());
            }
        }
        for child in &self.children {
            owners.extend(child.all_task_owners());
        }
        owners
    }

    /// Recursively collect all roadmap themes across the tree (union).
    pub fn all_roadmap_themes(&self) -> Themes {
        let mut themes = self.roadmap.clone();
        for child in &self.children {
            for theme in child.all_roadmap_themes().iter_spanned() {
                if !themes.contains(&theme.content) {
                    themes.push(theme.clone());
                }
            }
        }
        themes
    }

    /// Returns true if this node (or any child) matches the given roadmap theme.
    /// When children exist, only children are checked (they inherit parent themes).
    pub fn matches_roadmap_theme(&self, theme: &str) -> bool {
        if self.children.is_empty() {
            self.roadmap.contains(theme)
        } else {
            self.children.iter().any(|c| c.roadmap.contains(theme))
        }
    }
}

/// Deprecated: items required to complete the goal (pre-2026 format).
/// New goals use `TaskTree` instead.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoalPlan {
    /// If `Some`, title of the subsection in which these items were found.
    pub subgoal: Option<Spanned<String>>,

    /// List of items found in the table.
    pub plan_items: Vec<PlanItem>,
}

/// Deprecated: a row from the old-format (pre-2026) "Ownership and team asks" table.
/// New goals use `TaskItem` (via `TaskTree`) instead.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlanItem {
    pub text: Spanned<String>,
    pub owners: String,
    pub notes: String,
}

/// Returns the "owner(s)" of a plan-item, which can be
///
/// * users, if this is something that users have to do
/// * teams, if this is a team ask
#[derive(Debug)]
pub enum ParsedOwners {
    TeamAsks(Vec<&'static TeamName>),
    Usernames(Vec<String>),
}

/// Identifies a particular ask for a set of Rust teams.
///
/// This is the **old format** used before 2026. Each row in the "Ownership and team asks"
/// table specifies a task (like "RFC decision" or "Standard reviews") along with which
/// team(s) are being asked to do it, indicated by `![Team][]` in the owners column.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamAsk {
    /// Path to the markdown file containing this ask (appropriate for a link)
    pub link_path: Arc<PathBuf>,

    /// What the team is being asked for (e.g., RFC decision)
    pub ask_description: String,

    /// Title(s) of the goal. The first element is the title of the goal. The second, if present, is the subgoal.
    pub goal_titles: Vec<Spanned<String>>,

    /// Name(s) of the teams being asked to do the thing
    pub teams: Vec<&'static TeamName>,

    /// Owners of the subgoal or goal
    pub owners: String,

    /// Any notes
    pub notes: String,
}

/// The level of support needed from a team.
///
/// This is part of the **new format** introduced in 2026. Instead of listing specific
/// asks (like "RFC decision"), goals now specify a support level that indicates how
/// much involvement is needed from each team.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SupportLevel {
    /// Team only needs to do routine activities.
    /// Example: A compiler change that will require a few small PRs to be reviewed.
    Small,

    /// Dedicated support from one person, but the rest of the team doesn't have to do much.
    /// Example: Implementing a small, noncontroversial language feature.
    Medium,

    /// Deeper review from the entire team.
    /// Example: Rearchitecting part of the compiler or implementing a complex language feature.
    Large,
}

/// Represents the support needed from a single team for a goal.
///
/// This is the **new format** introduced in 2026. The "Team asks" section contains a table
/// with Team/Support level/Notes columns, rather than the old Task/Owner(s)/Notes format.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamSupport {
    /// Path to the markdown file containing this (appropriate for a link)
    pub link_path: Arc<PathBuf>,

    /// The team being asked for support
    pub team: &'static TeamName,

    /// Level of support needed (Small, Medium, Large)
    pub support_level: SupportLevel,

    /// Any notes about what's needed
    pub notes: String,

    /// If this came from a `###` subsection, the subgoal title
    pub subgoal: Option<Spanned<String>>,
}

/// How teams are involved with a goal - either through specific asks (old format)
/// or support levels (new format).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TeamInvolvement {
    /// Old format (pre-2026): specific asks like "RFC decision", "Standard reviews"
    /// Parsed from "Ownership and team asks" section with Task/Owner(s)/Notes columns.
    Asks(Vec<TeamAsk>),

    /// New format (2026+): support levels (Small, Medium, Large)
    /// Parsed from "Team asks" section with Team/Support level/Notes columns.
    Support(Vec<TeamSupport>),
}

impl TeamInvolvement {
    /// Returns all teams involved with this goal, regardless of format.
    pub fn teams(&self) -> BTreeSet<&'static TeamName> {
        match self {
            TeamInvolvement::Asks(asks) => asks.iter().flat_map(|a| &a.teams).copied().collect(),
            TeamInvolvement::Support(supports) => supports.iter().map(|s| s.team).collect(),
        }
    }

    /// Returns true if there is no team involvement.
    pub fn is_empty(&self) -> bool {
        match self {
            TeamInvolvement::Asks(asks) => asks.is_empty(),
            TeamInvolvement::Support(supports) => supports.is_empty(),
        }
    }

    /// Returns the team asks if this uses the old format, None otherwise.
    pub fn as_asks(&self) -> Option<&Vec<TeamAsk>> {
        match self {
            TeamInvolvement::Asks(asks) => Some(asks),
            TeamInvolvement::Support(_) => None,
        }
    }

    /// Returns the team support entries if this uses the new format, None otherwise.
    pub fn as_support(&self) -> Option<&Vec<TeamSupport>> {
        match self {
            TeamInvolvement::Asks(_) => None,
            TeamInvolvement::Support(supports) => Some(supports),
        }
    }
}

/// Load all the goals from a given directory.
/// Roadmap files (`roadmap-*.md`) are skipped; use `roadmaps_in_dir` for those.
pub fn goals_in_dir(directory_path: &Path) -> Result<Vec<GoalDocument>> {
    let mut goal_documents = vec![];
    for (path, link_path) in markdown_files(&directory_path)? {
        // Skip template files
        if path.file_name().unwrap() == "TEMPLATE.md" {
            continue;
        }

        // Skip roadmap files (they're loaded separately via roadmaps_in_dir)
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with("roadmap-") {
                continue;
            }
        }

        if let Some(goal_document) = GoalDocument::load(&path, &link_path)? {
            goal_documents.push(goal_document);
        }
    }
    Ok(goal_documents)
}

/// Load all the roadmaps from a given directory.
/// Only processes files matching the `roadmap-*.md` naming convention.
pub fn roadmaps_in_dir(directory_path: &Path) -> Result<Vec<RoadmapDocument>> {
    let mut roadmap_documents = vec![];
    for (path, link_path) in markdown_files(&directory_path)? {
        let Some(filename) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !filename.starts_with("roadmap-") {
            continue;
        }

        if let Some(roadmap_document) = RoadmapDocument::load(&path, &link_path)? {
            roadmap_documents.push(roadmap_document);
        }
    }
    Ok(roadmap_documents)
}

/// Validate that usernames are consistently capitalized across all goals.
/// GitHub usernames are case-insensitive, so `@BennoLossin` and `@bennolossin`
/// refer to the same person but would appear as duplicates in aggregated lists.
pub fn validate_username_consistency(goals: &[GoalDocument]) -> Result<()> {
    // Map from lowercase username to (set of observed casings, list of files where each appears)
    let mut seen: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, Vec<String>>,
    > = std::collections::BTreeMap::new();

    for goal in goals {
        let file = goal.path.display().to_string();

        // Collect usernames from point of contact
        for username in owner_usernames(&goal.metadata.pocs) {
            seen.entry(username.to_lowercase())
                .or_default()
                .entry(username.to_string())
                .or_default()
                .push(file.clone());
        }

        // Collect usernames from task owners (filter to @-prefixed entries,
        // since old-format goals may have non-username owner text)
        for username in &goal.task_owners {
            if !username.starts_with('@') {
                continue;
            }
            seen.entry(username.to_lowercase())
                .or_default()
                .entry(username.to_string())
                .or_default()
                .push(file.clone());
        }
    }

    let mut errors = Vec::new();
    for (_, casings) in &seen {
        if casings.len() > 1 {
            let variants: Vec<String> = casings
                .iter()
                .map(|(name, files)| format!("{} (in {})", name, files.join(", ")))
                .collect();
            errors.push(format!(
                "inconsistent username capitalization: {}",
                variants.join(" vs ")
            ));
        }
    }

    if !errors.is_empty() {
        spanned::bail_here!("{}", errors.join("\n"));
    }

    Ok(())
}

/// Validate that every `| Roadmap | theme |` declared by a goal has a corresponding
/// `roadmap-*.md` file whose short title matches. Skipped when no roadmap documents exist
/// in the directory (e.g. older milestones that used `| Flagship |`).
pub fn validate_roadmap_references(
    goals: &[GoalDocument],
    roadmaps: &[RoadmapDocument],
) -> Result<()> {
    if roadmaps.is_empty() {
        return Ok(());
    }

    let roadmap_titles: std::collections::BTreeSet<String> = roadmaps
        .iter()
        .map(|r| r.short_title.content.trim().to_string())
        .collect();

    for goal in goals {
        validate_themes(&goal.all_roadmaps(), &roadmap_titles)?;
    }

    Ok(())
}

/// Validate that every theme in `themes` has a corresponding roadmap document.
fn validate_themes(
    themes: &Themes,
    roadmap_titles: &std::collections::BTreeSet<String>,
) -> Result<()> {
    for theme in themes.iter_spanned() {
        let theme_name = theme.content.trim();
        if !roadmap_titles.contains(theme_name) {
            spanned::bail!(
                theme,
                "roadmap `{}` does not match any `roadmap-*.md` short title; \
                 available roadmaps: {}",
                theme_name,
                roadmap_titles
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }
    Ok(())
}

impl GoalDocument {
    fn load(path: &Path, link_path: &Path) -> Result<Option<Self>> {
        let sections = markwaydown::parse(path)?;

        let Some(metadata) = extract_metadata(&sections)? else {
            return Ok(None);
        };

        let summary = extract_summary(&sections)?;

        let link_path = Arc::new(link_path.to_path_buf());

        // Try to extract team involvement - could be old format or new format
        let (team_involvement, goal_plans, mut task_owners) =
            extract_team_involvement(&sections, &link_path, &metadata)?;

        // Enforce that every goal has some team involvement (unless it is not accepted)
        if metadata.status.is_not_not_accepted() && team_involvement.is_empty() {
            spanned::bail!(
                metadata.title,
                "no team involvement in goal; did you include a `Team asks` or `Ownership and team asks` section?"
            );
        }

        // Extract task tree from "Work items over the next year"
        let task_tree = extract_task_tree(&sections, &metadata)?;

        // Compute task_owners from the task tree
        task_owners.extend(task_tree.all_task_owners());

        Ok(Some(GoalDocument {
            path: path.to_path_buf(),
            link_path,
            summary: summary.unwrap_or_else(|| (*metadata.title).clone()),
            metadata,
            team_involvement,
            goal_plans,
            task_owners,
            task_tree,
        }))
    }

    /// Returns all teams involved with this goal.
    pub fn teams_with_asks(&self) -> BTreeSet<&'static TeamName> {
        self.team_involvement.teams()
    }

    /// True if this goal is a candidate (may yet be accepted)
    pub fn is_not_not_accepted(&self) -> bool {
        self.metadata.status.is_not_not_accepted()
    }

    /// Modify the goal document on disk to link to the given issue number in the metadata.
    pub fn link_issue(&self, number: IssueId) -> Result<()> {
        let mut metadata_table = self.metadata.table.clone();
        metadata_table
            .content
            .add_key_value_row(TRACKING_ISSUE_ROW, &number);

        Table::overwrite_in_path(&self.metadata.table, &self.path, &metadata_table)?;

        Ok(())
    }

    /// In goal lists, we render our point-of-contact as "Help Wanted" if this goal needs a contributor.
    pub fn point_of_contact_for_goal_list(&self) -> String {
        if self.metadata.is_help_wanted() {
            "![Help Wanted][]".to_string()
        } else {
            self.metadata.pocs.clone()
        }
    }

    /// Returns the "What and why" text for this goal.
    /// Falls back to the first sentence of the summary if no explicit "What and why" metadata row exists.
    pub fn what_and_why(&self) -> String {
        match &self.metadata.what_and_why {
            Some(w) => w.clone(),
            None => first_sentence(&self.summary),
        }
    }

    /// Returns true if this goal (or any task tree child) matches the given roadmap theme.
    pub fn matches_roadmap_theme(&self, theme: &str) -> bool {
        self.task_tree.matches_roadmap_theme(theme)
    }

    /// Returns all roadmap themes for this goal — the union across the task tree.
    pub fn all_roadmaps(&self) -> Themes {
        self.task_tree.all_roadmap_themes()
    }
}

/// Convert a heading title to an mdbook-compatible anchor slug.
fn slugify(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Generate progress HTML based on issue progress and state
fn generate_progress_html(progress: &Progress, state: &GithubIssueState) -> String {
    match (progress, state) {
        (Progress::Tracked { completed, total }, GithubIssueState::Closed) if completed == total => {
            r#"<img src="https://img.shields.io/badge/Completed!%20%3D%29-green" alt="Completed">"#.to_string()
        },
        (Progress::Tracked { completed: _, total: _ }, GithubIssueState::Closed) => {
            r#"<img src="https://img.shields.io/badge/Will%20not%20complete%20%3A%28-yellow" alt="Incomplete">"#.to_string()
        },
        (Progress::Tracked { completed, total }, _) => {
            format!(r#"<progress value="{}" max="{}">{}/{}</progress>"#, completed, total, completed, total)
        },
        (Progress::Binary { is_closed: true }, _) => {
            r#"<img src="https://img.shields.io/badge/Completed!%20%3D%29-green" alt="Completed">"#.to_string()
        },
        (Progress::Binary { is_closed: false }, _) => {
            r#"<progress value="0" max="1">0/1</progress>"#.to_string()
        },
        (Progress::Error { message }, _) => {
            format!(r#"<span title="{}">⚠️</span>"#, message)
        }
    }
}

pub fn format_goal_table(
    goals: &[&GoalDocument],
    milestone_issues: Option<&[ExistingGithubIssue]>,
) -> Result<String> {
    // If any of the goals have tracking issues, include those in the table.
    let show_champions = goals.iter().any(|g| {
        *g.metadata.status == Status::Proposed || *g.metadata.status == Status::NotAccepted
    });

    let mut table;

    if !show_champions {
        table = vec![vec![
            Spanned::here("Goal".to_string()),
            Spanned::here("Point of contact".to_string()),
            Spanned::here("Progress".to_string()),
        ]];

        for goal in goals {
            // Find the directory in which the goal document is located.
            // That is our "milestone" directory (e.g., 2024h2).
            let milestone: &str = goal
                .path
                .parent()
                .unwrap()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            let progress_bar = match &goal.metadata.tracking_issue {
                Some(
                    issue_id @ IssueId {
                        repository: Repository { org, repo },
                        number,
                    },
                ) => {
                    // Find the matching issue in milestone_issues and generate progress HTML
                    let progress_html = if let Some(issues) = milestone_issues {
                        if let Some(issue) = issues.iter().find(|issue| issue.number == *number) {
                            let progress = checkboxes(&issue);
                            generate_progress_html(&progress, &issue.state)
                        } else {
                            // Issue not found - might be in different milestone or not exist
                            r#"<span title="Issue not found in milestone">⚠️</span>"#.to_string()
                        }
                    } else {
                        // No milestone issues provided - fall back to empty div for now
                        format!("<div class='tracking-issue-progress' id='{milestone}:{org}:{repo}:{number}'></div>")
                    };

                    format!(
                        "<a href='{url}' alt='Tracking issue'>{progress_html}</a>",
                        url = issue_id.url(),
                        progress_html = progress_html
                    )
                }
                None => "(no tracking issue)".to_string(),
            };

            table.push(vec![
                Spanned::here(format!(
                    "[{}]({})",
                    *goal.metadata.title,
                    goal.link_path.display()
                )),
                Spanned::here(goal.point_of_contact_for_goal_list()),
                Spanned::here(progress_bar),
            ]);
        }
    } else {
        table = vec![vec![
            Spanned::here("Goal".to_string()),
            Spanned::here("Point of contact".to_string()),
            Spanned::here("Task Owners and Champions".to_string()),
        ]];

        for goal in goals {
            // Collect task owners, excluding those who are already the POC
            let mut contributors: Vec<String> = goal
                .task_owners
                .iter()
                .filter(|owner| !goal.metadata.pocs.contains(owner.as_str()))
                .cloned()
                .collect();

            // Collect champions with team affiliation
            let mut champions: Vec<String> = goal
                .metadata
                .champions
                .iter()
                .map(|(team, champion)| format!("{} ({})", champion.content, team))
                .collect();

            // Combine task owners and champions
            contributors.append(&mut champions);

            table.push(vec![
                Spanned::here(format!(
                    "[{}]({})",
                    *goal.metadata.title,
                    goal.link_path.display()
                )),
                Spanned::here(goal.point_of_contact_for_goal_list()),
                Spanned::here(contributors.join(", ")),
            ]);
        }
    }
    Ok(util::format_table(&table))
}

/// Format highlight goals as sections with people and summary at the given heading level.
pub fn format_highlight_goal_sections(
    goals: &[&GoalDocument],
    heading_level: usize,
) -> Result<String> {
    let mut output = String::new();
    let hashes = "#".repeat(heading_level);

    for goal in goals {
        if goal.metadata.is_help_wanted() {
            output.push_str(&format!(
                "{hashes} [{}]({}) ![Help wanted][]\n\n",
                *goal.metadata.title,
                goal.link_path.display()
            ));
        } else {
            output.push_str(&format!(
                "{hashes} [{}]({})\n\n",
                *goal.metadata.title,
                goal.link_path.display()
            ));
        }

        // Build people list: POC first (with role), then task owners (no role), then champions (with team role)
        let mut people: Vec<String> = Vec::new();

        people.push(format!("{} (point of contact)", goal.metadata.pocs));

        for owner in &goal.task_owners {
            if !goal.metadata.pocs.contains(owner.as_str()) {
                people.push(owner.clone());
            }
        }

        for (team, champion) in &goal.metadata.champions {
            people.push(format!("{} ({} champion)", champion.content, team));
        }

        if !people.is_empty() {
            output.push_str(&format!("*{}*\n\n", people.join(", ")));
        }
        output.push_str(goal.summary.trim());
        output.push_str("\n\n");
    }

    Ok(output)
}

/// Format highlight goals as a table with "Goal" and "What and why" columns.
/// When a goal has subgoals (task tree children), emits one row per child
/// instead of one row for the goal itself.
pub fn format_highlight_table(goals: &[&GoalDocument]) -> String {
    let mut output = String::new();
    output.push_str("| Goal | What and why |\n");
    output.push_str("| --- | --- |\n");
    for goal in goals {
        let children = &goal.task_tree.children;

        if children.is_empty() {
            output.push_str(&format!(
                "| [{}]({}) | {} |\n",
                *goal.metadata.title,
                goal.link_path.display(),
                goal.what_and_why(),
            ));
        } else {
            for child in children {
                let what_and_why = child
                    .what_and_why
                    .clone()
                    .unwrap_or_else(|| goal.what_and_why());
                let anchor = slugify(&child.title);
                output.push_str(&format!(
                    "| [{}]({}#{}) | {} |\n",
                    *child.title,
                    goal.link_path.display(),
                    anchor,
                    what_and_why,
                ));
            }
        }
    }

    output
}

/// Format a credits line listing all people involved in the given goals.
/// Unions task owners, POCs, and team champions, deduplicates, and sorts alphabetically.
pub fn format_highlight_credits(goals: &[&GoalDocument]) -> String {
    let mut all_people: BTreeSet<String> = BTreeSet::new();
    for goal in goals {
        for username in owner_usernames(&goal.metadata.pocs) {
            all_people.insert(username.to_string());
        }
        for owner in &goal.task_owners {
            if owner.starts_with('@') {
                all_people.insert(owner.clone());
            }
        }
        for (_team, champion) in &goal.metadata.champions {
            for username in owner_usernames(&champion.content) {
                all_people.insert(username.to_string());
            }
        }
    }

    if all_people.is_empty() {
        return String::new();
    }

    let people_list: Vec<&str> = all_people.iter().map(|s| s.as_str()).collect();
    people_list.join(", ")
}

/// Format roadmaps as a table with "Roadmap", "Point of contact", and "What and why" columns.
pub fn format_roadmap_table(roadmaps: &[&RoadmapDocument]) -> Result<String> {
    let mut table = vec![vec![
        Spanned::here("Roadmap".to_string()),
        Spanned::here("Point of contact".to_string()),
        Spanned::here("What and why".to_string()),
    ]];

    let mut sorted_roadmaps: Vec<&&RoadmapDocument> = roadmaps.iter().collect();
    sorted_roadmaps.sort_by_key(|r| &r.short_title);

    for roadmap in sorted_roadmaps {
        table.push(vec![
            Spanned::here(format!(
                "[{}]({})",
                *roadmap.short_title,
                roadmap.link_path.display()
            )),
            Spanned::here(roadmap.point_of_contact.clone()),
            Spanned::here(roadmap.what_and_why.clone()),
        ]);
    }

    Ok(util::format_table(&table))
}

/// Format matching goals as markdown table rows (no headers, no separator).
/// Each row has three columns: `| Goal | Timespan | What and why |`.
///
/// Used by the `(((ROADMAP ROWS: ...)))` directive, which appears
/// inside a manually authored table.
///
/// When a goal's task tree has children, emits one row per child that matches
/// `filter_theme` instead of one row for the goal.
pub fn format_roadmap_goal_rows(goals: &[&GoalDocument], filter_theme: &str) -> String {
    let mut output = String::new();
    for goal in goals {
        let milestone_dir = goal
            .path
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        let children = &goal.task_tree.children;

        if children.is_empty() {
            // No children: emit one row for the goal
            let timespan = goal.metadata.timespan.as_deref().unwrap_or(milestone_dir);
            output.push_str(&format!(
                "| [{}]({}) | {} | {} |\n",
                *goal.metadata.title,
                goal.link_path.display(),
                timespan,
                goal.what_and_why(),
            ));
        } else {
            // Has children: emit one row per child that matches the theme
            for child in children {
                if !child.roadmap.contains(filter_theme) {
                    continue;
                }
                let timespan = child.timespan.as_deref().unwrap_or(milestone_dir);
                let what_and_why = child
                    .what_and_why
                    .clone()
                    .unwrap_or_else(|| goal.what_and_why());
                let anchor = slugify(&child.title);
                output.push_str(&format!(
                    "| [{}]({}#{}) | {} | {} |\n",
                    *child.title,
                    goal.link_path.display(),
                    anchor,
                    timespan,
                    what_and_why,
                ));
            }
        }
    }
    output
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Status {
    Proposed,
    Accepted,
    NotAccepted,
}

impl Status {
    /// True if this goal has not yet been rejected
    pub fn is_not_not_accepted(&self) -> bool {
        *self != Status::NotAccepted
    }

    pub fn try_from(value: Spanned<&str>) -> Result<Spanned<Self>> {
        let value = value.trim();

        let valid_values = [
            ("Accepted", Status::Accepted),
            ("Proposed", Status::Proposed),
            ("Not accepted", Status::NotAccepted),
        ];

        for (valid_value, status) in valid_values {
            if value == valid_value {
                return Ok(value.map(|_| status));
            }
        }

        spanned::bail!(
            value,
            "unrecognized status, expected one of {:?}",
            valid_values.iter().map(|(s, _)| s).collect::<Vec<_>>(),
        )
    }
}

/// Parse all rows from a metadata table where the first column matches `key_name`
/// (case-insensitive), collecting column 1 values into a `Themes` set.
/// Markdown links like `[text](url)` are stripped to just the link text.
fn parse_themed_rows(table: &Table, key_name: &str) -> Themes {
    let mut themes = Themes::default();
    for row in &table.rows {
        if row[0].content.trim().eq_ignore_ascii_case(key_name) {
            let mut value = row[1].clone();
            if !value.content.trim().is_empty() {
                // Strip markdown links: `[Just add async](./roadmap-just-add-async.md)` → `Just add async`
                let stripped = crate::re::strip_markdown_link(&value.content);
                if stripped != value.content {
                    value.content = stripped.to_string();
                }
                themes.push(value);
            }
        }
    }
    themes
}

fn extract_metadata(sections: &[Section]) -> Result<Option<Metadata>> {
    let Some(first_section) = sections.first() else {
        return Ok(None);
    };

    if first_section.title.is_empty() {
        spanned::bail!(first_section.title, "first section has no title");
    }

    let title = &first_section.title;

    let Some(first_table) = first_section.tables.first() else {
        return Ok(None);
    };

    expect_headers(first_table, &["Metadata", ""])?;

    let short_title_row = first_table.rows.iter().find(|row| row[0] == "Short title");

    let Some(poc_row) = first_table
        .rows
        .iter()
        .find(|row| row[0] == "Point of contact")
    else {
        spanned::bail!(
            first_table.rows[0][0],
            "metadata table has no `Point of contact` row"
        )
    };

    if !re::is_just(&re::USERNAME, poc_row[1].trim()) {
        spanned::bail!(
            poc_row[1],
            "point of contact must be a single github username",
        )
    }

    let Some(status_row) = first_table.rows.iter().find(|row| row[0] == "Status") else {
        spanned::bail!(first_table.rows[0][0], "metadata table has no `Status` row")
    };

    let status = Status::try_from(status_row[1].as_deref())?;

    let issue = if let Some(r) = first_table
        .rows
        .iter()
        .find(|row| row[0] == TRACKING_ISSUE_ROW)
    {
        // Accepted goals must have a tracking issue.
        let has_tracking_issue = !r[1].is_empty();
        if *status == Status::Accepted && !has_tracking_issue {
            spanned::bail!(r[1], "accepted goals cannot have an empty tracking issue");
        }

        if has_tracking_issue && !r[1].contains("rust-project-goals#") {
            spanned::bail!(
                r[1],
                "tracking issues are issues in the rust-project-goals repository. \
                The `{}` issue can go in the `Other tracking issues` row.",
                r[1].as_str(),
            );
        }

        // For the others, it's of course optional.
        if has_tracking_issue {
            Some(r[1].parse()?.content)
        } else {
            None
        }
    } else {
        None
    };

    // We no longer require the Teams or Task owners rows to contain specific placeholders
    // since we auto-inject team names and task owners during preprocessing

    let mut champions = BTreeMap::default();
    for row in &first_table.rows {
        let row_name = &row[0];
        let row_value = &row[1];

        if !row_name.to_lowercase().contains("champion") {
            continue;
        }

        if let Some(m) = CHAMPION_METADATA.captures(row_name) {
            let team_name = m.name("team").unwrap().as_str().to_string();

            let Some(team) = team::get_team_name(&team_name)? else {
                spanned::bail!(row_name, "team `{team_name}` is not recognized")
            };

            if champions.contains_key(team) {
                spanned::bail!(
                    row_name,
                    "multiple rows naming champions for team `{team_name}`"
                )
            } else {
                champions.insert(team, row_value.clone());
            }
        } else {
            spanned::bail!(
                row_name,
                "metadata row `{}` talks about champions but is not of the form `[team-name] champion`",
                &**row_name
            )
        }
    }

    // Parse roadmap and highlight theme rows
    // Accept both "Roadmap" (new) and "Flagship" (old) for backward compatibility
    let mut roadmap = parse_themed_rows(first_table, "Roadmap");
    for theme in parse_themed_rows(first_table, "Flagship").iter() {
        roadmap.push(Spanned::here(theme.to_string()));
    }
    let highlight = parse_themed_rows(first_table, "Highlight");
    let needs = parse_themed_rows(first_table, "Needs");

    let what_and_why = first_table
        .rows
        .iter()
        .find(|row| row[0].content.trim().eq_ignore_ascii_case("What and why"))
        .map(|row| row[1].to_string());

    let timespan = first_table
        .rows
        .iter()
        .find(|row| row[0].content.trim().eq_ignore_ascii_case("Timespan"))
        .map(|row| row[1].to_string());

    Ok(Some(Metadata {
        title: title.clone(),
        short_title: if let Some(row) = short_title_row {
            row[1].clone()
        } else {
            title.clone()
        },
        pocs: poc_row[1].to_string(),
        status,
        tracking_issue: issue,
        table: first_table.clone(),
        champions,
        roadmap,
        highlight,
        needs,
        what_and_why,
        timespan,
    }))
}

/// Returns the first sentence of a string (up to and including the first `.` followed by whitespace or end of string).
/// Stops at blank lines (paragraph boundaries) and collapses internal newlines to spaces.
/// If no sentence boundary is found within the first paragraph, returns the whole first paragraph.
fn first_sentence(s: &str) -> String {
    let s = s.trim();

    // Take only the first paragraph (up to the first blank line)
    let first_para = s.split("\n\n").next().unwrap_or(s);

    // Collapse internal newlines to spaces
    let collapsed = first_para.replace('\n', " ");
    let collapsed = collapsed.trim();

    // Find the first sentence boundary (period followed by whitespace or end of string)
    let mut chars = collapsed.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch == '.' {
            let next_is_boundary = chars.peek().map_or(true, |&(_, next)| next.is_whitespace());
            if next_is_boundary {
                return collapsed[..=i].to_owned();
            }
        }
    }
    collapsed.to_owned()
}

pub fn extract_summary(sections: &[Section]) -> Result<Option<String>> {
    let Some(ownership_section) = sections.iter().find(|section| section.title == "Summary") else {
        return Ok(None);
    };

    Ok(Some(ownership_section.text.trim().to_string()))
}

struct RoadmapMetadata {
    title: Spanned<String>,
    short_title: Spanned<String>,
    what_and_why: String,
    point_of_contact: String,
}

/// Extract roadmap metadata from the first section of a markdown file.
/// Returns None if the file doesn't have the expected roadmap metadata structure.
fn extract_roadmap_metadata(sections: &[Section]) -> Result<Option<RoadmapMetadata>> {
    let Some(first_section) = sections.first() else {
        return Ok(None);
    };

    if first_section.title.is_empty() {
        spanned::bail!(first_section.title, "first section has no title");
    }

    let title = &first_section.title;

    let Some(first_table) = first_section.tables.first() else {
        return Ok(None);
    };

    expect_headers(first_table, &["Metadata", ""])?;

    let short_title = if let Some(row) = first_table.rows.iter().find(|row| row[0] == "Short title")
    {
        row[1].clone()
    } else {
        title.clone()
    };

    let Some(what_row) = first_table.rows.iter().find(|row| row[0] == "What and why") else {
        spanned::bail!(
            first_table.rows[0][0],
            "roadmap metadata table has no `What and why` row"
        )
    };

    let Some(poc_row) = first_table
        .rows
        .iter()
        .find(|row| row[0] == "Point of contact")
    else {
        spanned::bail!(
            first_table.rows[0][0],
            "roadmap metadata table has no `Point of contact` row"
        )
    };

    Ok(Some(RoadmapMetadata {
        title: title.clone(),
        short_title,
        what_and_why: what_row[1].to_string(),
        point_of_contact: poc_row[1].to_string(),
    }))
}

/// Extract team involvement from a goal document.
///
/// This function detects whether the goal uses the old format ("Ownership and team asks")
/// or the new format ("Team asks") and parses accordingly.
fn extract_team_involvement(
    sections: &[Section],
    link_path: &Arc<PathBuf>,
    metadata: &Metadata,
) -> Result<(TeamInvolvement, Vec<GoalPlan>, BTreeSet<String>)> {
    // Check for new format first (2026+): "Team asks" section
    if let Some(team_asks_index) = sections
        .iter()
        .position(|section| section.title == "Team asks")
    {
        let team_support = extract_team_support(sections, team_asks_index, link_path)?;
        // New format doesn't have goal_plans or task_owners in the same way
        return Ok((
            TeamInvolvement::Support(team_support),
            vec![],
            BTreeSet::new(),
        ));
    }

    // Fall back to old format (pre-2026): "Ownership and team asks" section
    if let Some(ownership_index) = sections
        .iter()
        .position(|section| section.title == "Ownership and team asks")
    {
        let goal_plans = extract_plan_items_from_index(sections, ownership_index)?;

        let mut team_asks = vec![];
        for goal_plan in &goal_plans {
            let mut goal_titles = vec![metadata.short_title.clone()];
            if let Some(subgoal) = &goal_plan.subgoal {
                goal_titles.push(subgoal.clone());
            }
            for plan_item in &goal_plan.plan_items {
                team_asks.extend(plan_item.team_asks(link_path, &goal_titles, &metadata.pocs)?);
            }
        }

        let task_owners = goal_plans
            .iter()
            .flat_map(|goal_plan| &goal_plan.plan_items)
            .flat_map(|plan_item| plan_item.task_owners())
            .collect();

        return Ok((TeamInvolvement::Asks(team_asks), goal_plans, task_owners));
    }

    if let Some(section) = sections.first() {
        spanned::bail!(
            &section.title,
            "no `Team asks` or `Ownership and team asks` section found"
        )
    } else {
        spanned::bail_here!(
            "no `Team asks` or `Ownership and team asks` section found in {}",
            link_path.display()
        )
    }
}

/// Extract team support entries from the new format (2026+).
fn extract_team_support(
    sections: &[Section],
    team_asks_index: usize,
    link_path: &Arc<PathBuf>,
) -> Result<Vec<TeamSupport>> {
    let section = &sections[team_asks_index];
    let level = section.level;

    let mut supports = vec![];

    // Extract from main section
    supports.extend(extract_team_support_from_section(None, section, link_path)?);

    // Extract from subsections (for subgoals)
    for subsection in sections
        .iter()
        .skip(team_asks_index + 1)
        .take_while(|s| s.level > level)
    {
        supports.extend(extract_team_support_from_section(
            Some(subsection.title.clone()),
            subsection,
            link_path,
        )?);
    }

    Ok(supports)
}

/// Extract team support entries from a single section's table.
fn extract_team_support_from_section(
    subgoal: Option<Spanned<String>>,
    section: &Section,
    link_path: &Arc<PathBuf>,
) -> Result<Vec<TeamSupport>> {
    if section.tables.is_empty() {
        return Ok(vec![]);
    }

    if section.tables.len() > 1 {
        spanned::bail!(
            section.title,
            "expected at most one table in Team asks section, found {}",
            section.tables.len()
        );
    }

    let table = &section.tables[0];
    expect_headers(table, &["Team", "Support level", "Notes"])?;

    let mut supports = vec![];
    for row in &table.rows {
        let team_str = row[0].trim();

        // Skip empty rows or placeholder rows
        if team_str.is_empty() || team_str == "..." {
            continue;
        }

        // Extract team name from markdown link format like [cargo] or [compiler]
        let team_names = extract_team_names(team_str);
        if team_names.is_empty() {
            spanned::bail!(row[0], "could not parse team name from `{}`", team_str);
        }

        let team_name = &team_names[0];
        let Some(team) = team::get_team_name(team_name)? else {
            let names = team::get_team_names()?;
            spanned::bail!(
                row[0],
                "no Rust team named `{}` found (valid names are {})",
                team_name,
                commas(names),
            );
        };

        let support_level_str = row[1].trim();

        // Skip rows with empty support level (template placeholders)
        if support_level_str.is_empty() {
            continue;
        }

        let support_level = SupportLevel::from_str(&row[1])?;

        supports.push(TeamSupport {
            link_path: link_path.clone(),
            team,
            support_level,
            notes: row[2].to_string(),
            subgoal: subgoal.clone(),
        });
    }

    Ok(supports)
}

impl SupportLevel {
    fn from_str(s: &Spanned<String>) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "small" => Ok(SupportLevel::Small),
            "medium" => Ok(SupportLevel::Medium),
            "large" => Ok(SupportLevel::Large),
            other => spanned::bail!(
                s,
                "unrecognized support level `{}`, expected one of: Small, Medium, Large",
                other
            ),
        }
    }

    /// Returns the display name for this support level.
    pub fn as_str(&self) -> &'static str {
        match self {
            SupportLevel::Small => "Small",
            SupportLevel::Medium => "Medium",
            SupportLevel::Large => "Large",
        }
    }
}

impl std::fmt::Display for SupportLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a goal's categorization by its maximum team ask level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalSize {
    /// At least one team has a Large ask
    Large,
    /// At least one team has a Medium ask, no Large asks
    Medium,
    /// Only Small asks
    Small,
}

impl GoalDocument {
    /// Returns the maximum support level across all team asks for this goal.
    /// Returns None if this goal uses the old format or has no team support entries.
    pub fn max_support_level(&self) -> Option<SupportLevel> {
        match &self.team_involvement {
            TeamInvolvement::Support(supports) => supports.iter().map(|s| s.support_level).max(),
            TeamInvolvement::Asks(_) => None,
        }
    }

    /// Categorizes the goal by its maximum team ask level.
    /// Returns None if this goal uses the old format.
    pub fn goal_size(&self) -> Option<GoalSize> {
        self.max_support_level().map(|level| match level {
            SupportLevel::Large => GoalSize::Large,
            SupportLevel::Medium => GoalSize::Medium,
            SupportLevel::Small => GoalSize::Small,
        })
    }
}

/// Format goals into tables with one row per team ask.
///
/// Output looks like:
///
/// ```markdown
/// | Goal | PoC | Team | Champion |
/// | :--- | :--- | :--- | :--- |
/// | [build-std](./build-std.md) | @davidtwco | **cargo** | TBD |
/// |  |  | compiler | *not needed* |
/// |  |  | libs | *not needed* |
/// ```
///
/// - Goal and PoC only appear on the first row for each goal
/// - Team name is **bold** for Large asks
/// - Champion shows `@username`, `TBD` (for Large/Medium), or `*not needed*` (for Small)
pub fn format_sized_goal_table(goals: &[&GoalDocument], size: GoalSize) -> Result<String> {
    use std::fmt::Write;

    // Filter to goals of the requested size
    let mut filtered_goals: Vec<_> = goals
        .iter()
        .filter(|g| g.goal_size() == Some(size))
        .collect();

    if filtered_goals.is_empty() {
        return Ok("*No goals in this category.*\n".to_string());
    }

    // Sort goals by title
    filtered_goals.sort_by(|a, b| a.metadata.title.cmp(&b.metadata.title));

    // Build the table
    let mut table: Vec<Vec<Spanned<String>>> = vec![vec![
        Spanned::here("Goal".to_string()),
        Spanned::here("PoC".to_string()),
        Spanned::here("Team".to_string()),
        Spanned::here("Champion".to_string()),
    ]];

    for goal in filtered_goals {
        // Get all team supports, sorted by level (Large first) then alphabetically
        let team_rows = goal_team_rows(goal);

        for (i, (team, level)) in team_rows.iter().enumerate() {
            let is_first_row = i == 0;

            // Goal and PoC only on first row
            let goal_cell = if is_first_row {
                format!(
                    "[{}]({})",
                    goal.metadata.short_title.content,
                    goal.link_path.display()
                )
            } else {
                String::new()
            };

            let poc_cell = if is_first_row {
                goal.point_of_contact_for_goal_list()
            } else {
                String::new()
            };

            // Team name: bold for Large, normal for Medium/Small
            let team_cell = if *level == SupportLevel::Large {
                format!("**{}**", team.name().to_lowercase())
            } else {
                team.name().to_lowercase()
            };

            // Champion: @username, ![TBD][], or *n/a*
            let champion_cell = if let Some(champion) = goal.metadata.champions.get(team) {
                champion.content.clone()
            } else {
                use SupportLevel::*;
                match level {
                    Large | Medium => "![TBD][]".to_string(),
                    Small => "*n/a*".to_string(),
                }
            };

            table.push(vec![
                Spanned::here(goal_cell),
                Spanned::here(poc_cell),
                Spanned::here(team_cell),
                Spanned::here(champion_cell),
            ]);
        }
    }

    let mut output = String::new();
    write!(output, "{}", util::format_table(&table))?;
    Ok(output)
}

/// Get all team supports for a goal, sorted by level (Large first) then alphabetically.
fn goal_team_rows(goal: &GoalDocument) -> Vec<(&'static TeamName, SupportLevel)> {
    let Some(supports) = goal.team_involvement.as_support() else {
        return vec![];
    };

    // Collect unique teams with their max support level
    let mut team_levels: BTreeMap<&'static TeamName, SupportLevel> = BTreeMap::new();
    for support in supports {
        team_levels
            .entry(support.team)
            .and_modify(|existing| {
                if support.support_level > *existing {
                    *existing = support.support_level;
                }
            })
            .or_insert(support.support_level);
    }

    // Sort by level (Large > Medium > Small) then by team name
    let mut sorted: Vec<_> = team_levels.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name().cmp(&b.0.name())));

    sorted
}

/// Extract a task tree from "Work items over the next year".
/// Returns an empty tree (no tasks, no children) if the section is absent.
fn extract_task_tree(sections: &[Section], metadata: &Metadata) -> Result<TaskTree> {
    let Some(work_items_index) = sections
        .iter()
        .position(|s| s.title.content.trim() == "Work items over the next year")
    else {
        return Ok(TaskTree::root(metadata, vec![], vec![]));
    };

    let work_items_section = &sections[work_items_index];
    let work_items_level = work_items_section.level;

    // Parse task table at the root level (if present)
    let root_tasks = extract_task_items(work_items_section)?;

    // Collect child sections exactly one level deeper (#### under ###)
    let subsections: Vec<&Section> = sections
        .iter()
        .skip(work_items_index + 1)
        .take_while(|s| s.level > work_items_level)
        .filter(|s| s.level == work_items_level + 1)
        .collect();

    let mut children = vec![];
    for subsection in subsections {
        children.push(parse_task_tree_child(subsection, metadata)?);
    }

    Ok(TaskTree::root(metadata, root_tasks, children))
}

/// Parse a single `####` child node of the task tree.
/// Looks for an optional `| Metadata | |` table and an optional `| Task | Owner(s) | Notes |` table.
fn parse_task_tree_child(section: &Section, parent_metadata: &Metadata) -> Result<TaskTree> {
    // Look for a metadata table (headers: ["Metadata", ""])
    let metadata_table = section
        .tables
        .iter()
        .find(|t| t.header.len() == 2 && t.header[0].content.trim() == "Metadata");

    // Parse child-specific fields from metadata table (if present)
    let (own_roadmap, own_timespan, own_what_and_why) = if let Some(table) = metadata_table {
        let roadmap = parse_themed_rows(table, "Roadmap");
        let timespan = table
            .rows
            .iter()
            .find(|row| row[0].content.trim().eq_ignore_ascii_case("Timespan"))
            .map(|row| row[1].to_string());
        let what_and_why = table
            .rows
            .iter()
            .find(|row| row[0].content.trim().eq_ignore_ascii_case("What and why"))
            .map(|row| row[1].to_string());
        (roadmap, timespan, what_and_why)
    } else {
        (Themes::default(), None, None)
    };

    // Compute effective roadmap: parent UNION child's own
    let mut effective_roadmap = parent_metadata.roadmap.clone();
    for theme in own_roadmap.iter_spanned() {
        if !effective_roadmap.contains(&theme.content) {
            effective_roadmap.push(theme.clone());
        }
    }

    // Compute effective timespan: child's own > parent's
    let effective_timespan = own_timespan.or_else(|| parent_metadata.timespan.clone());

    // Compute effective what_and_why: metadata table > first sentence of section text > parent's
    let own_what_and_why = own_what_and_why.or_else(|| {
        let text = section.text.trim();
        if text.is_empty() {
            None
        } else {
            Some(first_sentence(text))
        }
    });
    let effective_what_and_why = own_what_and_why.or_else(|| parent_metadata.what_and_why.clone());

    // Parse task items from this section
    let tasks = extract_task_items(section)?;

    Ok(TaskTree {
        title: section.title.clone(),
        tasks,
        roadmap: effective_roadmap,
        timespan: effective_timespan,
        what_and_why: effective_what_and_why,
        children: vec![],
    })
}

/// Extract task items from a section's `| Task | Owner(s) | Notes |` table (if present).
fn extract_task_items(section: &Section) -> Result<Vec<TaskItem>> {
    // Look for a task table with 3 columns: Task, Owner(s) [or team(s)], Notes
    let task_table = section.tables.iter().find(|t| {
        t.header.len() == 3
            && t.header[0].content.trim() == "Task"
            && t.header[2].content.trim() == "Notes"
    });

    let Some(table) = task_table else {
        return Ok(vec![]);
    };

    let mut items = vec![];
    for row in &table.rows {
        items.push(TaskItem {
            task: row[0].clone(),
            owners: row[1].to_string(),
            notes: row[2].to_string(),
        });
    }

    Ok(items)
}

/// Extract plan items from the old format, starting at the given section index.
fn extract_plan_items_from_index(
    sections: &[Section],
    ownership_index: usize,
) -> Result<Vec<GoalPlan>> {
    // Extract the plan items from the main section (if any)
    let level = sections[ownership_index].level;

    let mut goal_plans = vec![];
    goal_plans.extend(goal_plan(None, &sections[ownership_index])?);

    for subsection in sections
        .iter()
        .skip(ownership_index + 1)
        .take_while(|s| s.level > level)
    {
        goal_plans.extend(goal_plan(Some(subsection.title.clone()), subsection)?);
    }

    Ok(goal_plans)
}

fn goal_plan(subgoal: Option<Spanned<String>>, section: &Section) -> Result<Option<GoalPlan>> {
    match section.tables.len() {
        0 => Ok(None),
        1 => {
            let table = &section.tables[0];
            expect_headers(table, &["Task", "Owner(s) or team(s)", "Notes"])?;

            let mut rows = table.rows.iter().peekable();
            let mut plan_items = vec![];
            while rows.peek().is_some() {
                plan_items.push(extract_plan_item(&mut rows)?);
            }

            Ok(Some(GoalPlan {
                subgoal,
                plan_items,
            }))
        }
        thats_too_many => {
            let mut table_error = Vec::new();
            for (idx, table) in section.tables.iter().enumerate() {
                let header: Vec<_> = table.header.iter().map(|h| h.to_string()).collect();
                table_error.push(format!("{}: {:?}", idx + 1, header.join(", ")));
            }

            spanned::bail!(
                section.title,
                "markdown parsing unexpectedly encountered multiple ({}) goal tables in section `{}`:\n{}",
                thats_too_many,
                section.title.content,
                table_error.join("\n"),
            )
        }
    }
}

fn extract_plan_item(
    rows: &mut std::iter::Peekable<std::slice::Iter<Vec<Spanned<String>>>>,
) -> Result<PlanItem> {
    let Some(row) = rows.next() else {
        spanned::bail_here!("unexpected end of table");
    };

    Ok(PlanItem {
        text: row[0].clone(),
        owners: row[1].to_string(),
        notes: row[2].to_string(),
    })
}

impl PlanItem {
    /// Parses the owners of this plan item.
    pub fn parse_owners(&self) -> Result<Option<ParsedOwners>> {
        if self.owners.is_empty() {
            Ok(None)
        } else if self.is_team_ask() {
            Ok(Some(ParsedOwners::TeamAsks(self.teams_being_asked()?)))
        } else {
            Ok(Some(ParsedOwners::Usernames(
                owner_usernames(&self.owners)
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            )))
        }
    }

    /// True if the plan item is noted as being completed
    pub fn is_complete(&self) -> bool {
        self.notes.contains("![Complete]")
    }

    /// If true, this item is something being asked of a team.
    /// If false, it's something the goal owner(s) are proposing to do.
    pub fn is_team_ask(&self) -> bool {
        self.owners.contains("![Team]")
    }

    /// Return the set of teams being asked to do things by this item, or empty vector if this is not a team ask.
    pub fn teams_being_asked(&self) -> Result<Vec<&'static TeamName>> {
        if !self.is_team_ask() {
            return Ok(vec![]);
        }

        let mut teams = vec![];
        for team_name in extract_team_names(&self.owners) {
            let Some(team) = team::get_team_name(&team_name)? else {
                let names = team::get_team_names()?;
                spanned::bail!(
                    self.text,
                    "no Rust team named `{}` found (valid names are {})",
                    team_name,
                    commas(names),
                );
            };

            teams.push(team);
        }

        if teams.is_empty() {
            spanned::bail!(
                self.text,
                "team ask for \"{}\" does not list any teams",
                self.text.content
            );
        }

        Ok(teams)
    }

    /// Extract the usernames of task owners from the `owners` field.
    /// Just treat it like a comma-separated list.
    fn task_owners(&self) -> Vec<String> {
        if self.is_team_ask() {
            return vec![];
        }

        self.owners
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Return a vector of all the team-asks from this item and its children.
    /// Invoked during `GoalDocument`.
    ///
    /// # Parameters
    ///
    /// * `link_path`, the path to the document this plan item is found within
    /// * `goal_title`, the title of the goal (or subgoal) this plan item is a part of
    /// * `goal_owners`, the owners of the goal (or subgoal) this plan item is a part of
    fn team_asks(
        &self,
        link_path: &Arc<PathBuf>,
        goal_titles: &Vec<Spanned<String>>,
        goal_owners: &str,
    ) -> Result<Vec<TeamAsk>> {
        let mut asks = vec![];

        let teams = self.teams_being_asked()?;
        if !teams.is_empty() {
            let config = Configuration::get();
            if !config.team_asks.contains_key(&*self.text) {
                return Err(
                    Error::new_str(self.text.as_ref().map(|_| "unrecognized team ask")).wrap_str(
                        Spanned::here(format!(
                            "team asks must be one of the following:\n{}",
                            config
                                .team_asks
                                .iter()
                                .map(|(ask, TeamAskDetails { about, .. })| {
                                    format!("* {ask:?}, meaning team should {about}")
                                })
                                .collect::<Vec<_>>()
                                .join("\n")
                        )),
                    ),
                );
            }

            asks.push(TeamAsk {
                link_path: link_path.clone(),
                ask_description: self.text.content.clone(),
                goal_titles: goal_titles.clone(),
                teams,
                owners: goal_owners.to_string(),
                notes: self.notes.clone(),
            });
        }

        Ok(asks)
    }
}

fn expect_headers(table: &Table, expected: &[&str]) -> Result<()> {
    if table.header != expected {
        // FIXME: do a diff so we see which headers are missing or extraneous

        return Err(
            Error::new_str(table.header[0].as_ref().map(|_| "unexpected table header")).wrap_str(
                Spanned::here(format!(
                    "expected `{expected:?}`, found `{:?}`",
                    table.header.iter().map(|h| &h.content),
                )),
            ),
        );
    }

    Ok(())
}

fn extract_team_names(s: &str) -> Vec<String> {
    extract_identifiers(s)
        .into_iter()
        .filter(|&s| s != "Team")
        .map(|s| s.to_string())
        .collect()
}

fn extract_identifiers(s: &str) -> Vec<&str> {
    re::IDENTIFIERS.find_iter(s).map(|m| m.as_str()).collect()
}

impl Metadata {
    /// Returns true if this goal has any highlight themes.
    pub fn is_highlight(&self) -> bool {
        self.highlight.is_some()
    }

    /// Extracts the `@abc` usernames found in the owner listing.
    pub fn owner_usernames(&self) -> Vec<&str> {
        owner_usernames(&self.pocs)
    }
}

fn owner_usernames(text: &str) -> Vec<&str> {
    text.split(char::is_whitespace)
        .filter_map(|owner| re::USERNAME.captures(owner))
        .map(|captures| captures.get(0).unwrap().as_str())
        .collect()
}

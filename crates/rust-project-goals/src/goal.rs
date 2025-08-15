use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::{collections::BTreeSet, path::PathBuf};

use regex::Regex;
use spanned::{Error, Result, Spanned};

use crate::config::{Configuration, TeamAskDetails};
use crate::gh::issue_id::{IssueId, Repository};
use crate::markwaydown::{self, Section, Table};
use crate::re::{self, CHAMPION_METADATA};
use crate::team::{self, TeamName};
use crate::util::{self, commas, markdown_files};

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

    /// The "plan" for completing the goal (includes things owners will do as well as team asks)
    pub goal_plans: Vec<GoalPlan>,

    /// Owners of any task that are not team asks.
    pub task_owners: BTreeSet<String>,

    /// List of team asks extracted from the goal
    pub team_asks: Vec<TeamAsk>,
}

/// Metadata loaded from the goal header
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    #[allow(unused)]
    pub title: String,
    pub short_title: Spanned<String>,
    pub pocs: String,
    pub status: Spanned<Status>,
    pub tracking_issue: Option<IssueId>,
    pub table: Spanned<Table>,

    /// For each table entry like `[T-lang] champion`, we create an entry in this map
    pub champions: BTreeMap<&'static TeamName, Spanned<String>>,

    /// Flagship category, if this is a flagship goal
    pub flagship: Option<Spanned<String>>,
}

pub const TRACKING_ISSUE_ROW: &str = "Tracking issue";

/// Items required to complete the goal.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoalPlan {
    /// If `Some`, title of the subsection in which these items were found.
    pub subgoal: Option<Spanned<String>>,

    /// List of items found in the table.
    pub plan_items: Vec<PlanItem>,
}

/// Identifies a particular ask for a set of Rust teams
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

/// Identifies a particular ask for a set of Rust teams
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

/// Load all the goals from a given directory
pub fn goals_in_dir(directory_path: &Path) -> Result<Vec<GoalDocument>> {
    let mut goal_documents = vec![];
    for (path, link_path) in markdown_files(&directory_path)? {
        // Skip template files
        if path.file_name().unwrap() == "TEMPLATE.md" {
            continue;
        }

        if let Some(goal_document) = GoalDocument::load(&path, &link_path)? {
            goal_documents.push(goal_document);
        }
    }
    Ok(goal_documents)
}

impl GoalDocument {
    fn load(path: &Path, link_path: &Path) -> Result<Option<Self>> {
        let sections = markwaydown::parse(path)?;

        let Some(metadata) = extract_metadata(&sections)? else {
            return Ok(None);
        };

        let summary = extract_summary(&sections)?;

        let link_path = Arc::new(link_path.to_path_buf());

        let goal_plans = if metadata.status.is_not_not_accepted() {
            extract_plan_items(&sections)?
        } else {
            vec![]
        };

        let mut team_asks = vec![];
        for goal_plan in &goal_plans {
            let mut goal_titles = vec![metadata.short_title.clone()];
            if let Some(subgoal) = &goal_plan.subgoal {
                goal_titles.push(subgoal.clone());
            }
            for plan_item in &goal_plan.plan_items {
                team_asks.extend(plan_item.team_asks(&link_path, &goal_titles, &metadata.pocs)?);
            }
        }

        // Enforce that every goal has some team asks (unless it is not accepted)
        if metadata.status.is_not_not_accepted() && team_asks.is_empty() {
            spanned::bail_here!("no team asks in goal; did you include `![Team]` in the table?");
        }

        let task_owners = goal_plans
            .iter()
            .flat_map(|goal_plan| &goal_plan.plan_items)
            .flat_map(|plan_item| plan_item.task_owners())
            .collect();

        Ok(Some(GoalDocument {
            path: path.to_path_buf(),
            link_path,
            summary: summary.unwrap_or_else(|| metadata.title.clone()),
            metadata,
            team_asks,
            goal_plans,
            task_owners,
        }))
    }

    pub fn teams_with_asks(&self) -> BTreeSet<&'static TeamName> {
        self.team_asks
            .iter()
            .flat_map(|ask| &ask.teams)
            .copied()
            .collect()
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
        self.metadata
            .table
            .overwrite_in_path(&self.path, &metadata_table)?;
        Ok(())
    }

    /// In goal lists, we render our point-of-contact as "Help Wanted" if this is an invited goal.
    pub fn point_of_contact_for_goal_list(&self) -> String {
        if self.metadata.status.is_invited {
            "![Help Wanted][]".to_string()
        } else {
            self.metadata.pocs.clone()
        }
    }
}

pub fn format_goal_table(goals: &[&GoalDocument]) -> Result<String> {
    // If any of the goals have tracking issues, include those in the table.
    let goals_are_proposed = goals
        .iter()
        .any(|g| g.metadata.status.acceptance == AcceptanceStatus::Proposed);

    let mut table;

    if !goals_are_proposed {
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
                Some(issue_id @ IssueId { repository: Repository { org, repo }, number }) => format!(
                    "<a href='{url}' alt='Tracking issue'><div class='tracking-issue-progress' id='{milestone}:{org}:{repo}:{number}'></div></a>",
                    url = issue_id.url(),
                ),
                None => format!("(no tracking issue)"),
            };

            table.push(vec![
                Spanned::here(format!(
                    "[{}]({})",
                    goal.metadata.title,
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
            Spanned::here("Team(s) and Champion(s)".to_string()),
        ]];

        for goal in goals {
            let teams: BTreeSet<&TeamName> = goal
                .team_asks
                .iter()
                .flat_map(|ask| &ask.teams)
                .copied()
                .collect();

            // Format teams with champions in parentheses
            let teams_with_champions: Vec<String> = teams
                .into_iter()
                .map(|team| {
                    if let Some(champion) = goal.metadata.champions.get(team) {
                        format!("{} ({})", team, champion.content)
                    } else {
                        team.to_string()
                    }
                })
                .collect();

            table.push(vec![
                Spanned::here(format!(
                    "[{}]({})",
                    goal.metadata.title,
                    goal.link_path.display()
                )),
                Spanned::here(goal.point_of_contact_for_goal_list()),
                Spanned::here(teams_with_champions.join(", ")),
            ]);
        }
    }
    Ok(util::format_table(&table))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Status {
    /// If true, this is a flagship goal (or a flagship candidate)
    pub is_flagship: bool,

    pub acceptance: AcceptanceStatus,

    /// If true, this is an INVITED goal, meaning that it lacks a primary owner
    pub is_invited: bool,
}

impl Status {
    /// True if this goal has not yet been rejected
    pub fn is_not_not_accepted(&self) -> bool {
        self.acceptance != AcceptanceStatus::NotAccepted
    }

    pub fn try_from(value: Spanned<&str>) -> Result<Spanned<Self>> {
        let value = value.trim();

        let valid_values = [
            (
                "Flagship",
                Status {
                    is_flagship: true,
                    acceptance: AcceptanceStatus::Accepted,
                    is_invited: false,
                },
            ),
            (
                "Accepted",
                Status {
                    is_flagship: false,
                    acceptance: AcceptanceStatus::Accepted,
                    is_invited: false,
                },
            ),
            (
                "Invited",
                Status {
                    is_flagship: false,
                    acceptance: AcceptanceStatus::Accepted,
                    is_invited: true,
                },
            ),
            (
                "Proposed",
                Status {
                    is_flagship: false,
                    acceptance: AcceptanceStatus::Proposed,
                    is_invited: false,
                },
            ),
            (
                "Proposed for flagship",
                Status {
                    is_flagship: true,
                    acceptance: AcceptanceStatus::Proposed,
                    is_invited: false,
                },
            ),
            (
                "Proposed for mentorship",
                Status {
                    is_flagship: false,
                    acceptance: AcceptanceStatus::Proposed,
                    is_invited: true,
                },
            ),
            (
                "Not accepted",
                Status {
                    is_flagship: false,
                    acceptance: AcceptanceStatus::NotAccepted,
                    is_invited: false,
                },
            ),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum AcceptanceStatus {
    Proposed,
    Accepted,
    NotAccepted,
}

fn extract_metadata(sections: &[Section]) -> Result<Option<Metadata>> {
    let Some(first_section) = sections.first() else {
        spanned::bail_here!("no markdown sections found in input")
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
        if status.acceptance == AcceptanceStatus::Accepted && !has_tracking_issue {
            spanned::bail!(r[1], "accepted goals cannot have an empty tracking issue");
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

    // Parse flagship row if present
    let flagship = first_table
        .rows
        .iter()
        .find(|row| row[0] == "Flagship")
        .map(|row| row[1].clone());

    Ok(Some(Metadata {
        title: title.to_string(),
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
        flagship,
    }))
}

fn extract_summary(sections: &[Section]) -> Result<Option<String>> {
    let Some(ownership_section) = sections.iter().find(|section| section.title == "Summary") else {
        return Ok(None);
    };

    Ok(Some(ownership_section.text.trim().to_string()))
}

fn extract_plan_items<'i>(sections: &[Section]) -> Result<Vec<GoalPlan>> {
    let Some(ownership_index) = sections
        .iter()
        .position(|section| section.title == "Ownership and team asks")
    else {
        spanned::bail_here!("no `Ownership and team asks` section found")
    };

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

    if goal_plans.is_empty() {
        spanned::bail!(
            sections[ownership_index].title,
            "no goal table items found in the `Ownership and team asks` section or subsections"
        )
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
    let regex = Regex::new("[-.A-Za-z]+").unwrap();
    regex.find_iter(s).map(|m| m.as_str()).collect()
}

impl Metadata {
    /// Returns the flagship category if this is a flagship goal
    pub fn flagship(&self) -> Option<&str> {
        self.flagship.as_ref().map(|s| s.content.as_str())
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

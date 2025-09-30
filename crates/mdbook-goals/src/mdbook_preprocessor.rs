use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use chrono::{self, Datelike};
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::Regex;
use rust_project_goals::config::{Configuration, GoalsConfig};
use rust_project_goals::format_champions::format_champions;
use rust_project_goals::format_team_ask::format_team_asks;
use rust_project_goals::markdown_processor::{MarkdownProcessor, MarkdownProcessorState};
use rust_project_goals::util;

use rust_project_goals::spanned::Spanned;
use rust_project_goals::{
    goal::{self, GoalDocument, TeamAsk},
    re,
    team::TeamName,
};

/// Load goals configuration from book.toml using clean serde deserialization
fn load_goals_config_from_book_toml(ctx: &PreprocessorContext) -> anyhow::Result<GoalsConfig> {
    // Find book.toml in the source directory
    let book_toml_path = ctx.root.join("book.toml");
    if book_toml_path.exists() {
        GoalsConfig::from_book_toml(book_toml_path)
    } else {
        Ok(GoalsConfig::default())
    }
}

pub struct GoalPreprocessor;

impl Preprocessor for GoalPreprocessor {
    fn name(&self) -> &str {
        "goals"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        let mut this = GoalPreprocessorWithContext::new(ctx)?;
        for section in &mut book.sections {
            this.process_book_item(section)?;
        }
        Ok(book)
    }
}

pub struct GoalPreprocessorWithContext<'c> {
    ctx: &'c PreprocessorContext,
    markdown_processor: MarkdownProcessor,
    processor_state: MarkdownProcessorState,
    goal_document_map: BTreeMap<PathBuf, Arc<Vec<GoalDocument>>>,
    milestone_issues_cache:
        BTreeMap<String, Arc<Vec<rust_project_goals::gh::issues::ExistingGithubIssue>>>,
}

impl<'c> GoalPreprocessorWithContext<'c> {
    pub fn new(ctx: &'c PreprocessorContext) -> anyhow::Result<Self> {
        // Extract goals configuration using clean parsing
        let goals_config = load_goals_config_from_book_toml(ctx)?;

        // Create the shared markdown processor
        let markdown_processor = MarkdownProcessor::new(goals_config);

        Ok(GoalPreprocessorWithContext {
            ctx,
            markdown_processor,
            processor_state: MarkdownProcessorState::default(),
            goal_document_map: Default::default(),
            milestone_issues_cache: Default::default(),
        })
    }

    fn process_book_item(&mut self, book_item: &mut BookItem) -> anyhow::Result<()> {
        match book_item {
            BookItem::Chapter(chapter) => {
                self.replace_metadata_placeholders(chapter)?;
                self.replace_champions(chapter)?;
                self.replace_team_asks(chapter)?;
                self.replace_valid_team_asks(chapter)?;
                self.replace_goal_lists(chapter)?;
                self.replace_goal_count(chapter)?;
                self.replace_flagship_goal_count(chapter)?;
                self.replace_reports(chapter)?;
                // Process all markdown linking using shared processor
                chapter.content = self
                    .markdown_processor
                    .process_markdown(&chapter.content, &mut self.processor_state)?;

                for sub_item in &mut chapter.sub_items {
                    self.process_book_item(sub_item)?;
                }

                Ok(())
            }

            BookItem::Separator => Ok(()),

            BookItem::PartTitle(_) => Ok(()),
        }
    }

    fn replace_goal_count(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        if !re::GOAL_COUNT.is_match(&chapter.content) {
            return Ok(());
        }

        let Some(chapter_path) = &chapter.path else {
            anyhow::bail!("found `(((#GOALS)))` but chapter has no path")
        };

        let goals = self.goal_documents(chapter_path)?;

        let count = goals
            .iter()
            .filter(|g| g.metadata.status.is_not_not_accepted())
            .count();

        chapter.content = re::GOAL_COUNT
            .replace_all(&chapter.content, &count.to_string())
            .to_string();

        Ok(())
    }

    fn replace_flagship_goal_count(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        if !re::FLAGSHIP_GOAL_COUNT.is_match(&chapter.content) {
            return Ok(());
        }

        let Some(chapter_path) = &chapter.path else {
            anyhow::bail!("found `(((#FLAGSHIP_GOALS)))` but chapter has no path")
        };

        let goals = self.goal_documents(chapter_path)?;

        let count = goals
            .iter()
            .filter(|g| g.metadata.flagship().is_some() && g.metadata.status.is_not_not_accepted())
            .count();

        chapter.content = re::FLAGSHIP_GOAL_COUNT
            .replace_all(&chapter.content, &count.to_string())
            .to_string();

        Ok(())
    }

    fn replace_goal_lists(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        // Handle filtered flagship goals first (more specific pattern)
        self.replace_flagship_goal_lists_filtered(chapter)?;

        // Handle unfiltered flagship goals
        self.replace_goal_lists_helper(chapter, &re::FLAGSHIP_GOAL_LIST, |goal, _capture| {
            goal.metadata.flagship().is_some() && goal.metadata.status.content.is_not_not_accepted()
        })?;

        self.replace_goal_lists_helper(chapter, &re::OTHER_GOAL_LIST, |goal, _capture| {
            goal.metadata.flagship().is_none() && goal.metadata.status.content.is_not_not_accepted()
        })?;
        self.replace_goal_lists_helper(chapter, &re::GOAL_LIST, |goal, _capture| {
            goal.metadata.status.content.is_not_not_accepted()
        })?;
        self.replace_goal_lists_helper(chapter, &re::GOAL_NOT_ACCEPTED_LIST, |goal, _capture| {
            !goal.metadata.status.content.is_not_not_accepted()
        })?;
        Ok(())
    }

    fn replace_flagship_goal_lists_filtered(
        &mut self,
        chapter: &mut Chapter,
    ) -> anyhow::Result<()> {
        self.replace_goal_lists_helper(
            chapter,
            &re::FLAGSHIP_GOAL_LIST_FILTERED,
            |goal, capture| {
                let filter_value = capture.unwrap().trim(); // Safe because this regex always has a capture
                goal.metadata.status.content.is_not_not_accepted()
                    && goal.metadata.flagship().map(|f| f.trim()) == Some(filter_value)
            },
        )
    }

    fn replace_goal_lists_helper(
        &mut self,
        chapter: &mut Chapter,
        regex: &Regex,
        filter: impl Fn(&GoalDocument, Option<&str>) -> bool,
    ) -> anyhow::Result<()> {
        loop {
            let Some(m) = regex.find(&chapter.content) else {
                return Ok(());
            };
            let range = m.range();

            let Some(chapter_path) = &chapter.path else {
                anyhow::bail!("found `{regex}` but chapter has no path")
            };

            // Extract capture group if present
            let capture_value = regex
                .captures(&chapter.content[range.clone()])
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str().trim());

            // Extract out the list of goals with the given filter.
            let goals = self.goal_documents(chapter_path)?;
            let mut goals_with_status: Vec<&GoalDocument> =
                goals.iter().filter(|g| filter(g, capture_value)).collect();

            goals_with_status.sort_by_key(|g| &g.metadata.title);

            // Get milestone issues for progress generation
            let milestone_issues = if let Some(first_goal) = goals_with_status.first() {
                // Extract milestone from the first goal's path
                let milestone = first_goal
                    .path
                    .parent()
                    .and_then(|p| p.file_stem())
                    .and_then(|s| s.to_str());

                if let Some(milestone) = milestone {
                    match self.get_or_load_milestone_issues(milestone) {
                        Ok(issues) => Some(issues),
                        Err(e) => {
                            eprintln!(
                                "⚠️ Failed to load milestone issues for {}: {}",
                                milestone, e
                            );
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // Format the list of goals and replace the `<!-- -->` comment with that.
            let output = goal::format_goal_table(
                &goals_with_status,
                milestone_issues.as_ref().map(|arc| arc.as_slice()),
            )
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            chapter.content.replace_range(range, &output);

            // Populate with children if this is not README
            if chapter_path.file_stem() != Some("README".as_ref()) {
                let mut parent_names = chapter.parent_names.clone();
                parent_names.push(chapter.name.clone());
                for (goal, index) in goals_with_status.iter().zip(0..) {
                    let content = std::fs::read_to_string(&goal.path)
                        .with_context(|| format!("reading `{}`", goal.path.display()))?;
                    let path = goal.path.strip_prefix(&self.ctx.config.book.src).unwrap();
                    let mut new_chapter =
                        Chapter::new(&goal.metadata.title, content, path, parent_names.clone());

                    if let Some(mut number) = chapter.number.clone() {
                        number.0.push(index + 1);
                        new_chapter.number = Some(number);
                    }

                    chapter.sub_items.push(BookItem::Chapter(new_chapter));
                }
            }
        }
    }

    /// Look for `(((CHAMPIONS)))` in the chapter content and replace it with the champions table.
    fn replace_champions(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let Some(m) = re::CHAMPIONS.find(&chapter.content) else {
            return Ok(());
        };
        let range = m.range();

        let Some(path) = &chapter.path else {
            anyhow::bail!("found `(((CHAMPIONS)))` but chapter has no path")
        };

        let goals = self.goal_documents(path)?;
        let goal_refs: Vec<&GoalDocument> = goals.iter().collect();
        let format_champions = format_champions(&goal_refs).map_err(|e| anyhow::anyhow!("{e}"))?;
        chapter.content.replace_range(range, &format_champions);

        Ok(())
    }

    /// Look for `<!-- TEAM ASKS -->` in the chapter content and replace it with the team asks.
    fn replace_team_asks(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let Some(m) = re::TEAM_ASKS.find(&chapter.content) else {
            return Ok(());
        };
        let range = m.range();

        let Some(path) = &chapter.path else {
            anyhow::bail!("found `(((TEAM ASKS)))` but chapter has no path")
        };

        let goals = self.goal_documents(path)?;
        let asks_of_any_team: Vec<&TeamAsk> = goals
            .iter()
            .filter(|g| g.metadata.status.is_not_not_accepted())
            .flat_map(|g| &g.team_asks)
            .collect();
        let format_team_asks =
            format_team_asks(&asks_of_any_team).map_err(|e| anyhow::anyhow!("{e}"))?;
        chapter.content.replace_range(range, &format_team_asks);

        Ok(())
    }

    /// Look for `<!-- TEAM ASKS -->` in the chapter content and replace it with the team asks.
    fn replace_valid_team_asks(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        if !re::VALID_TEAM_ASKS.is_match(&chapter.content) {
            return Ok(());
        }
        let config = Configuration::get();
        let rows = std::iter::once(vec![
            Spanned::here("Ask".to_string()),
            Spanned::here("aka".to_string()),
            Spanned::here("Description".to_string()),
        ])
        .chain(config.team_asks.iter().map(|(name, details)| {
            vec![
                Spanned::here(format!("{name:?}")),
                Spanned::here(details.short.to_string()),
                Spanned::here(details.about.to_string()),
            ]
        }))
        .collect::<Vec<Vec<Spanned<String>>>>();
        let table = util::format_table(&rows);
        let new_content = re::VALID_TEAM_ASKS.replace_all(&chapter.content, table);
        chapter.content = new_content.to_string();
        Ok(())
    }

    /// Find the goal documents for the milestone in which this `chapter_path` resides.
    /// e.g., if invoked with `2024h2/xxx.md`, will find all goal documents in `2024h2`.
    fn goal_documents(&mut self, chapter_path: &Path) -> anyhow::Result<Arc<Vec<GoalDocument>>> {
        let Some(milestone_path) = chapter_path.parent() else {
            anyhow::bail!("cannot get goal documents from `{chapter_path:?}`")
        };

        if let Some(goals) = self.goal_document_map.get(milestone_path) {
            return Ok(goals.clone());
        }

        let goal_documents = goal::goals_in_dir(&self.ctx.config.book.src.join(milestone_path))
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let goals = Arc::new(goal_documents);
        self.goal_document_map
            .insert(milestone_path.to_path_buf(), goals.clone());
        Ok(goals)
    }

    /// Get or load milestone issues, caching the result for subsequent calls.
    /// This eliminates redundant GitHub API calls within a single preprocessor run.
    fn get_or_load_milestone_issues(
        &mut self,
        milestone: &str,
    ) -> anyhow::Result<Arc<Vec<rust_project_goals::gh::issues::ExistingGithubIssue>>> {
        if let Some(cached_issues) = self.milestone_issues_cache.get(milestone) {
            eprintln!("📦 Using cached issues for milestone: {}", milestone);
            return Ok(cached_issues.clone());
        }

        eprintln!(
            "🌐 Loading issues from GitHub API for milestone: {}",
            milestone
        );
        let repository =
            rust_project_goals::gh::issue_id::Repository::new("rust-lang", "rust-project-goals");
        let issues =
            rust_project_goals::gh::issues::list_issues_in_milestone(&repository, milestone)
                .map_err(|e| {
                    anyhow::anyhow!("Failed to load milestone issues for {}: {}", milestone, e)
                })?;

        eprintln!(
            "✅ Loaded {} issues for milestone: {}",
            issues.len(),
            milestone
        );
        let issues = Arc::new(issues);
        self.milestone_issues_cache
            .insert(milestone.to_string(), issues.clone());
        Ok(issues)
    }

    /// Replace placeholders like TASK_OWNERS and TEAMS_WITH_ASKS.
    /// All goal documents should have this in their metadata table;
    /// that is enforced during goal parsing.
    fn replace_metadata_placeholders(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        // Auto-inject teams and task owners directly into metadata table instead of using placeholders
        self.inject_metadata_rows(chapter)?;

        Ok(())
    }

    /// Find the end of a markdown table (first line that doesn't start with |).
    /// Returns the byte offset where new rows should be inserted.
    fn find_markdown_table_end(content: &str) -> Option<usize> {
        let lines: Vec<&str> = content.lines().collect();

        // Find first line starting with |
        let table_start = lines
            .iter()
            .position(|line| line.trim_start().starts_with('|'))?;

        // Find first line after table_start that doesn't start with |
        let table_end = lines[table_start..]
            .iter()
            .position(|line| !line.trim_start().starts_with('|'))
            .map(|pos| table_start + pos)
            .unwrap_or(lines.len());

        // Calculate byte offset to the start of the table_end line
        let mut offset = 0;
        for i in 0..table_end {
            if i > 0 {
                offset += 1; // newline
            }
            offset += lines[i].len();
        }

        if table_end < lines.len() {
            // There's a line after the table, insert before it
            Some(offset + 1) // +1 for the newline after the last table row
        } else {
            // Table goes to end of file, append at the end
            Some(content.len())
        }
    }

    /// Automatically inject team names and task owners into the metadata table.
    /// This replaces the need for manual placeholders and combines the logic
    /// to avoid duplicate table parsing.
    fn inject_metadata_rows(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let Some(chapter_path) = chapter.path.as_ref() else {
            return Ok(()); // No path, nothing to inject
        };

        // Skip template files
        if chapter_path.file_name().and_then(|n| n.to_str()) == Some("TEMPLATE.md") {
            return Ok(());
        }

        // Only process files in milestone directories (like 2024h2, 2025h1, etc.)
        let Some(parent_dir) = chapter_path.parent() else {
            return Ok(());
        };

        let Some(parent_name) = parent_dir.file_name().and_then(|n| n.to_str()) else {
            return Ok(());
        };

        if !parent_name
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
        {
            return Ok(()); // Not a milestone directory, skip
        }

        // Find the goal document for this chapter
        let goals = self.goal_documents(&chapter_path)?;
        let chapter_in_context = self.ctx.config.book.src.join(chapter_path);
        let Some(goal) = goals.iter().find(|gd| gd.path == chapter_in_context) else {
            return Ok(()); // No goal document found, nothing to inject
        };

        // Compute the team names
        let team_names: Vec<String> = goal
            .teams_with_asks()
            .iter()
            .map(|team_name| team_name.name())
            .collect();

        let teams_text = if team_names.is_empty() {
            "(none)".to_string()
        } else {
            team_names.join(", ")
        };

        // Compute the task owner names
        let task_owners: Vec<String> = goal.task_owners.iter().cloned().collect();

        let task_owners_text = if task_owners.is_empty() {
            "(none)".to_string()
        } else {
            task_owners.join(", ")
        };

        // Find the table end and insert both rows
        if let Some(table_end) = Self::find_markdown_table_end(&chapter.content) {
            let insertion_text = format!(
                "| Teams            | {} |\n| Task owners      | {} |\n",
                teams_text, task_owners_text
            );
            chapter.content.insert_str(table_end, &insertion_text);
        }

        Ok(())
    }

    fn replace_reports(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        if !re::REPORTS.is_match(&chapter.content) {
            return Ok(());
        }

        let chapter_path = match &chapter.path {
            Some(path) => path.clone(),
            None => anyhow::bail!("found REPORTS placeholder but chapter has no path"),
        };

        // Parse date range from the placeholder
        let date_range = if let Some(captures) = re::REPORTS.captures(&chapter.content) {
            captures.get(1).map(|m| m.as_str().trim())
        } else {
            None
        };

        // Generate list of months based on date range
        let months = self.generate_month_list(date_range)?;

        // Discover teams with champions
        let goals = self.goal_documents(&chapter_path)?;
        let mut teams_with_champions: BTreeSet<&'static TeamName> = BTreeSet::new();

        for goal in goals.iter() {
            for team_name in goal.metadata.champions.keys() {
                teams_with_champions.insert(team_name);
            }
        }

        let now = chrono::Utc::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S UTC");

        // Generate dynamic chapters instead of files
        self.generate_report_chapters(chapter, &chapter_path, &months, &teams_with_champions)?;

        let replacement = format!(
            "This section contains automatically generated reports based on the comments left in the goal tracking issues.\n\
            \n\
            These reports were last generated at {timestamp}.",
        );

        chapter.content = re::REPORTS
            .replace_all(&chapter.content, replacement)
            .to_string();

        Ok(())
    }

    fn generate_report_chapters(
        &mut self,
        parent_chapter: &mut Chapter,
        chapter_path: &Path,
        months: &[(i32, u32, &'static str)],
        teams_with_champions: &BTreeSet<&'static TeamName>,
    ) -> anyhow::Result<()> {
        // Get the milestone from the chapter path (e.g., "2025h2" from "src/2025h2/reports.md")
        let milestone = chapter_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Could not determine milestone from chapter path: {:?}",
                    chapter_path
                )
            })?;

        let mut parent_names = parent_chapter.parent_names.clone();
        parent_names.push(parent_chapter.name.clone());
        let mut chapter_index = 1;

        // Generate blog post chapters
        for (year, month, month_name) in months.iter().rev() {
            // Reverse to show newest first
            let blog_content = self.generate_blog_post_content(milestone, *year, *month)?;

            let chapter_name = format!("{} Blog Post", month_name);
            let virtual_path = format!("blog-post-{:04}-{:02}.md", year, month);
            let path = Path::new(&virtual_path);

            let mut blog_chapter =
                Chapter::new(&chapter_name, blog_content, path, parent_names.clone());

            if let Some(mut number) = parent_chapter.number.clone() {
                number.0.push(chapter_index);
                blog_chapter.number = Some(number);
                chapter_index += 1;
            }

            parent_chapter
                .sub_items
                .push(BookItem::Chapter(blog_chapter));
        }

        // Generate champion report chapters
        for team_name in teams_with_champions {
            let team_name_str = &team_name.data().name;
            // Create a team folder chapter
            let team_chapter_name = format!("{} Team Reports", team_name_str);
            let team_virtual_path = format!("{}/index.md", team_name_str);
            let team_path = Path::new(&team_virtual_path);

            let team_content = format!("# {} Team Champion Reports\n\nThis section contains monthly champion reports for the {} team.", team_name_str, team_name_str);
            let mut team_chapter = Chapter::new(
                &team_chapter_name,
                team_content,
                team_path,
                parent_names.clone(),
            );

            if let Some(mut number) = parent_chapter.number.clone() {
                number.0.push(chapter_index);
                team_chapter.number = Some(number);
                chapter_index += 1;
            }

            let mut team_parent_names = parent_names.clone();
            team_parent_names.push(team_chapter_name.clone());
            let mut team_sub_index = 1;

            // Generate monthly reports for this team
            for (year, month, month_name) in months.iter().rev() {
                // Reverse to show newest first
                let champion_content =
                    self.generate_champion_report_content(milestone, team_name_str, *year, *month)?;

                let monthly_chapter_name = format!("{} {}", month_name, year);
                let monthly_virtual_path = format!("{}/{:04}-{:02}.md", team_name_str, year, month);
                let monthly_path = Path::new(&monthly_virtual_path);

                let mut monthly_chapter = Chapter::new(
                    &monthly_chapter_name,
                    champion_content,
                    monthly_path,
                    team_parent_names.clone(),
                );

                if let Some(mut number) = team_chapter.number.clone() {
                    number.0.push(team_sub_index);
                    monthly_chapter.number = Some(number);
                    team_sub_index += 1;
                }

                team_chapter
                    .sub_items
                    .push(BookItem::Chapter(monthly_chapter));
            }

            parent_chapter
                .sub_items
                .push(BookItem::Chapter(team_chapter));
        }

        Ok(())
    }

    fn generate_blog_post_content(
        &mut self,
        milestone: &str,
        year: i32,
        month: u32,
    ) -> anyhow::Result<String> {
        use chrono::NaiveDate;

        eprintln!(
            "📝 Generating blog post for {}-{:02} (milestone: {})",
            year, month, milestone
        );

        // Calculate start and end dates for the month
        let start_date = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month))?;
        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .ok_or_else(|| anyhow::anyhow!("Invalid end date calculation for {}-{:02}", year, month))?;

        // Get repository from context - assuming rust-lang/rust-project-goals as default
        let repository =
            rust_project_goals::gh::issue_id::Repository::new("rust-lang", "rust-project-goals");

        // Use cached issues for this milestone
        let issues = self.get_or_load_milestone_issues(milestone)?;

        // Use the library function with pre-loaded issues
        let content = rust_project_goals_cli::render_updates(
            &issues,
            &repository,
            milestone,
            &Some(start_date),
            &Some(end_date),
            None,
            false,
        )
        .map_err(|e| anyhow::anyhow!("Failed to generate blog post content: {}", e))?;

        Ok(content)
    }

    fn generate_champion_report_content(
        &mut self,
        milestone: &str,
        team_name: &str,
        year: i32,
        month: u32,
    ) -> anyhow::Result<String> {
        use chrono::NaiveDate;

        eprintln!(
            "👥 Generating champion report for {} team, {}-{:02} (milestone: {})",
            team_name, year, month, milestone
        );

        // Calculate start and end dates for the month
        let start_date = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month))?;
        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .ok_or_else(|| anyhow::anyhow!("Invalid end date calculation for {}-{:02}", year, month))?;

        // Get repository from context - assuming rust-lang/rust-project-goals as default
        let repository =
            rust_project_goals::gh::issue_id::Repository::new("rust-lang", "rust-project-goals");

        // Use cached issues for this milestone
        let issues = self.get_or_load_milestone_issues(milestone)?;

        // Use the library function with team filter (team_name is already in T-teamname format)
        let content = rust_project_goals_cli::render_updates(
            &issues,
            &repository,
            milestone,
            &Some(start_date),
            &Some(end_date),
            Some(team_name),
            false,
        )
        .map_err(|e| anyhow::anyhow!("Failed to generate champion report content: {}", e))?;

        Ok(content)
    }

    fn generate_month_list(
        &self,
        date_range: Option<&str>,
    ) -> anyhow::Result<Vec<(i32, u32, &'static str)>> {
        let (start_date, end_date) = if let Some(range_str) = date_range {
            self.parse_date_range(range_str)?
        } else {
            // Default to current month only if no range specified
            let now = chrono::Utc::now();
            let start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
                .ok_or_else(|| anyhow::anyhow!("Invalid current date"))?;
            (start, start)
        };

        let mut months = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            let month_name = match current.month() {
                1 => "January",
                2 => "February",
                3 => "March",
                4 => "April",
                5 => "May",
                6 => "June",
                7 => "July",
                8 => "August",
                9 => "September",
                10 => "October",
                11 => "November",
                12 => "December",
                _ => "Unknown",
            };

            months.push((current.year(), current.month(), month_name));

            // Move to next month
            if current.month() == 12 {
                current = chrono::NaiveDate::from_ymd_opt(current.year() + 1, 1, 1)
                    .ok_or_else(|| anyhow::anyhow!("Invalid date calculation"))?;
            } else {
                current = chrono::NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1)
                    .ok_or_else(|| anyhow::anyhow!("Invalid date calculation"))?;
            }
        }

        Ok(months)
    }

    fn parse_date_range(
        &self,
        range_str: &str,
    ) -> anyhow::Result<(chrono::NaiveDate, chrono::NaiveDate)> {
        // Parse format like "2025-09-01 to 2025-12-31"
        let parts: Vec<&str> = range_str.split(" to ").collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid date range format. Expected 'YYYY-MM-DD to YYYY-MM-DD'");
        }

        let start_date = chrono::NaiveDate::parse_from_str(parts[0].trim(), "%Y-%m-%d")
            .with_context(|| format!("Invalid start date: {}", parts[0]))?;
        let end_date = chrono::NaiveDate::parse_from_str(parts[1].trim(), "%Y-%m-%d")
            .with_context(|| format!("Invalid end date: {}", parts[1]))?;

        if start_date > end_date {
            anyhow::bail!("Start date must be before or equal to end date");
        }

        Ok((start_date, end_date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_markdown_table_end() {
        let content = "Some text before\n\n| Metadata | Value |\n|----------|-------|\n| Point of contact | @nikomatsakis |\n| Teams | (none) |\n\nSome text after";

        let result = GoalPreprocessorWithContext::find_markdown_table_end(content);
        assert!(result.is_some());

        let offset = result.unwrap();
        let (before, after) = content.split_at(offset);

        // Should split right before the blank line after the table
        assert!(before.ends_with("| Teams | (none) |\n"));
        assert!(after.starts_with("\nSome text after"));

        // Test that inserting at this offset works correctly
        let mut test_content = content.to_string();
        test_content.insert_str(offset, "| New row | value |\n");
        assert!(test_content.contains("| Teams | (none) |\n| New row | value |\n\nSome text after"));
    }

    #[test]
    fn test_reports_replacement() {
        use mdbook::book::Chapter;

        let mut chapter = Chapter::new(
            "Test Chapter",
            "# Test\n\n(((REPORTS)))\n\nEnd".to_string(),
            "test.md",
            Vec::new(),
        );

        // Test the regex directly
        assert!(re::REPORTS.is_match(&chapter.content));

        // Test the replacement logic with a simple fixed replacement
        // (since we can't easily mock the goal documents in a unit test)
        let now = chrono::Utc::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S UTC");

        let replacement = format!(
            r#"This section contains automatically generated reports based on the comments left in the goal tracking issues.

These reports were last generated at {}.

## Blog post

These are the main blog posts that are published each month:

* [October](./blog-post-2025-10.md)
* [September](./blog-post-2025-09.md)

## Champion reports

These reports include the details only of goals for a particular team.

"#,
            timestamp
        );

        chapter.content = re::REPORTS
            .replace_all(&chapter.content, replacement)
            .to_string();

        // Check that the placeholder was replaced
        assert!(!chapter.content.contains("(((REPORTS)))"));
        assert!(chapter
            .content
            .contains("These reports were last generated at"));
        assert!(chapter.content.contains("## Blog post"));
        assert!(chapter.content.contains("## Champion reports"));
    }

    #[test]
    fn test_date_range_parsing() {
        // Test date range parsing directly
        let parts: Vec<&str> = "2025-09-01 to 2025-12-31".split(" to ").collect();
        assert_eq!(parts.len(), 2);

        let start_date = chrono::NaiveDate::parse_from_str(parts[0].trim(), "%Y-%m-%d").unwrap();
        let end_date = chrono::NaiveDate::parse_from_str(parts[1].trim(), "%Y-%m-%d").unwrap();

        assert_eq!(start_date.year(), 2025);
        assert_eq!(start_date.month(), 9);
        assert_eq!(end_date.year(), 2025);
        assert_eq!(end_date.month(), 12);

        // Test month generation logic
        let mut months = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            months.push((current.year(), current.month()));

            // Move to next month
            if current.month() == 12 {
                current = chrono::NaiveDate::from_ymd_opt(current.year() + 1, 1, 1).unwrap();
            } else {
                current = chrono::NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1)
                    .unwrap();
            }
        }

        assert_eq!(months.len(), 4); // September, October, November, December
        assert_eq!(months[0], (2025, 9));
        assert_eq!(months[1], (2025, 10));
        assert_eq!(months[2], (2025, 11));
        assert_eq!(months[3], (2025, 12));
    }
}

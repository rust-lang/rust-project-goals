use std::path::{Path, PathBuf};

use handlebars::{DirectorySourceOptions, Handlebars};
use rust_project_goals::gh::issues::ExistingGithubComment;
use serde::Serialize;

use rust_project_goals_json::Progress;

pub struct Templates<'h> {
    reg: Handlebars<'h>,
}

impl<'h> Templates<'h> {
    pub fn new() -> anyhow::Result<Self> {
        let templates = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../templates");
        Self::from_templates_dir(&templates)
    }

    pub fn from_templates_dir(dir_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let dir_path = dir_path.as_ref();
        let mut reg = Handlebars::new();

        reg.set_strict_mode(true);

        reg.register_templates_directory(dir_path, DirectorySourceOptions::default())?;
        assert!(reg.get_template("updates").is_some());

        reg.register_helper("markdown_to_html", Box::new(markdown_to_html));
        reg.register_helper("is_complete", Box::new(is_complete));

        Ok(Templates { reg })
    }
}

handlebars::handlebars_helper!(markdown_to_html: |md: String| comrak::markdown_to_html(&md, &Default::default()));

handlebars::handlebars_helper!(is_complete: |p: Progress| match p {
    Progress::Binary { is_closed } => is_closed,
    Progress::Tracked { completed, total } => completed == total,
    Progress::Error { .. } => false,
});

/// The parameters expected by the `updates.md` template.
#[derive(Serialize, Debug)]
pub struct Updates {
    pub milestone: String,
    pub flagship_goals: Vec<UpdatesGoal>,
    pub other_goals: Vec<UpdatesGoal>,
    pub goal_count: usize,
    pub flagship_goal_count: usize,
}

impl Updates {
    pub fn new(
        milestone: String,
        flagship_goals: Vec<UpdatesGoal>,
        other_goals: Vec<UpdatesGoal>,
    ) -> Self {
        Updates {
            milestone,
            flagship_goal_count: flagship_goals.len(),
            goal_count: flagship_goals.len() + other_goals.len(),
            flagship_goals,
            other_goals,
        }
    }
    pub fn render(self) -> anyhow::Result<String> {
        let templates = Templates::new()?;
        Ok(templates.reg.render("updates", &self)?)
    }
}

/// Part of the parameters expected by the `updates.md` template.
#[derive(Serialize, Debug)]
pub struct UpdatesGoal {
    /// Title of the tracking issue
    pub title: String,

    /// Tracking issue number on the project goals repository
    pub issue_number: u64,

    /// Comma-separated list of assignees
    pub issue_assignees: String,

    /// URL of the tracking issue
    pub issue_url: String,

    /// True if the issue is closed.
    pub is_closed: bool,

    /// True if there are "help wanted" comments OR the TL;DR includes a help wanted request.
    pub has_help_wanted: bool,

    /// If there are comments that include ["help wanted"](`rust_project_goals::re::HELP_WANTED`)
    /// comments, those comments are included here.
    pub help_wanted: Vec<HelpWanted>,

    /// Markdown with update text (bullet list)
    pub comments: Vec<ExistingGithubComment>,

    /// Comments.len but accessible to the template
    pub num_comments: usize,

    /// Progress towards the goal
    pub progress: Progress,

    /// TL;DR comment (if any, empty string if none)
    pub tldr: Option<String>,

    /// Contents of a "Why this goal?" section in the tracking issue (empty string if not present)
    pub why_this_goal: String,
}

#[derive(Serialize, Debug)]
pub struct HelpWanted {
    pub text: String,
}

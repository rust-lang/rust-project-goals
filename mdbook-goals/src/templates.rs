use std::path::{Path, PathBuf};

use handlebars::{DirectorySourceOptions, Handlebars};
use serde::Serialize;

use crate::json::Progress;

pub struct Templates<'h> {
    reg: Handlebars<'h>,
}

impl<'h> Templates<'h> {
    pub fn new() -> anyhow::Result<Self> {
        let templates = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../templates");
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
    pub flagship_goals: Vec<UpdatesFlagshipGoal>,
    pub other_goals_with_updates: Vec<UpdatesOtherGoal>,
    pub other_goals_without_updates: Vec<UpdatesOtherGoal>,
}

impl Updates {
    pub fn render(self) -> anyhow::Result<String> {
        let templates = Templates::new()?;
        Ok(templates.reg.render("updates", &self)?)
    }
}

/// Part of the parameters expected by the `updates.md` template.
#[derive(Serialize, Debug)]
pub struct UpdatesFlagshipGoal {
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

    /// Progress towards the goal
    pub progress: Progress,

    /// Updates provided towards the goal
    pub updates: Vec<UpdatesFlagshipGoalUpdate>,
}

/// Part of the parameters expected by the `updates.md` template.
#[derive(Serialize, Debug)]
pub struct UpdatesFlagshipGoalUpdate {
    /// Username of the person who wrote the update
    pub author: String,

    /// Formatted like "Oct 26"
    pub date: String,

    /// Text of the update
    pub update: String,

    /// URL of the comment
    pub url: String,
}

/// Part of the parameters expected by the `updates.md` template.
#[derive(Serialize, Debug)]
pub struct UpdatesOtherGoal {
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

    /// Markdown with update text (bullet list)
    pub updates_markdown: String,

    /// Progress towards the goal
    pub progress: Progress,
}

//! Generate JSON summarizing the tracking issues.
//!
//! This module contains types (e.g., [`TrackingIssues`]) that represent the
//! external API that is used by the website
//! and other tools to consume the tracking issue data. They are very similar
//! to the types in `gh` and so forth but because they represent
//! a versioned API, we copy them over here to insulate them from incidental changes.

use std::path::PathBuf;

use serde::Serialize;

use crate::{
    gh::{
        issue_id::Repository,
        issues::{
            list_issue_titles_in_milestone, ExistingGithubComment, ExistingGithubIssue,
            ExistingIssueState,
        },
    },
    re,
};

pub(super) fn generate_json(
    repository: &Repository,
    milestone: &str,
    json_path: &Option<PathBuf>,
) -> anyhow::Result<()> {
    let issues = list_issue_titles_in_milestone(repository, milestone)?;

    let issues = TrackingIssues {
        issues: issues
            .into_iter()
            .map(|(title, issue)| {
                let (total_checkboxes, checked_checkboxes) = checkboxes(&issue);
                TrackingIssue {
                    number: issue.number,
                    title,
                    flagship: is_flagship(&issue),
                    total_checkboxes,
                    checked_checkboxes,
                    assignees: issue.assignees.into_iter().collect(),
                    updates: updates(issue.comments),
                    state: issue.state,
                }
            })
            .collect(),
        repository: repository.to_string(),
        milestone: milestone.to_string(),
    };

    if let Some(json_path) = json_path {
        let json = serde_json::to_string(&issues)?;
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(json_path, json)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&issues)?);
    }

    Ok(())
}

#[derive(Serialize)]
struct TrackingIssues {
    repository: String,
    milestone: String,
    issues: Vec<TrackingIssue>,
}

#[derive(Serialize)]
struct TrackingIssue {
    /// Issue number on the repository
    number: u64,

    /// Title of the tracking issue
    title: String,

    /// True if this is a flagship goal
    flagship: bool,

    /// Total checkboxes appearing in the body (i.e., `* [ ]` or `* [x]`)
    total_checkboxes: u32,

    /// Checked checkboxes appearing in the body (i.e., `* [x]`)
    checked_checkboxes: u32,

    /// Set of assigned people
    assignees: Vec<String>,

    /// Posts that we consider to be status updates, in chronological order
    updates: Vec<TrackingIssueUpdate>,

    /// Issue state
    state: ExistingIssueState,
}

#[derive(Serialize)]
struct TrackingIssueUpdate {
    pub author: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub url: String,
}

fn checkboxes(issue: &ExistingGithubIssue) -> (u32, u32) {
    let mut total = 0;
    let mut checked = 0;

    for line in issue.body.lines() {
        if re::CHECKBOX.is_match(line) {
            total += 1;
        }

        if re::CHECKED_CHECKBOX.is_match(line) {
            checked += 1;
        }
    }

    (total, checked)
}

fn is_flagship(issue: &ExistingGithubIssue) -> bool {
    issue.labels.iter().any(|label| label.name == "flagship")
}

fn updates(comments: Vec<ExistingGithubComment>) -> Vec<TrackingIssueUpdate> {
    comments
        .into_iter()
        .filter(|comment| !comment.is_automated_comment())
        .map(|comment| TrackingIssueUpdate {
            author: comment.author,
            body: comment.body,
            created_at: comment.created_at,
            url: comment.url,
        })
        .collect()
}

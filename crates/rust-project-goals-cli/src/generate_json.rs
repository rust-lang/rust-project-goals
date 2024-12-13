use std::path::PathBuf;

use rust_project_goals::gh::{
    issue_id::Repository,
    issues::{checkboxes, list_issue_titles_in_milestone, ExistingGithubComment},
};
use rust_project_goals_json::{TrackingIssue, TrackingIssueUpdate, TrackingIssues};

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
                let progress = checkboxes(&issue);
                TrackingIssue {
                    number: issue.number,
                    title,
                    flagship: issue.has_flagship_label(),
                    progress,
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

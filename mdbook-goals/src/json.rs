//! Generate JSON summarizing the tracking issues.
//!
//! This module contains types (e.g., [`TrackingIssues`]) that represent the
//! external API that is used by the website
//! and other tools to consume the tracking issue data. They are very similar
//! to the types in `gh` and so forth but because they represent
//! a versioned API, we copy them over here to insulate them from incidental changes.

use std::{path::PathBuf, str::FromStr};

use serde::Serialize;

use crate::{
    gh::{
        issue_id::Repository,
        issues::{
            count_issues_matching_search, fetch_issue, list_issue_titles_in_milestone, CountIssues,
            ExistingGithubComment, ExistingGithubIssue, ExistingIssueState,
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

    /// State of progress
    progress: Progress,

    /// Set of assigned people
    assignees: Vec<String>,

    /// Posts that we consider to be status updates, in chronological order
    updates: Vec<TrackingIssueUpdate>,

    /// Issue state
    state: ExistingIssueState,
}

#[derive(Serialize, Debug)]
pub enum Progress {
    /// We could not find any checkboxes or other deatils on the tracking issue.
    /// So all we have is "open" or "closed".
    Binary {
        is_closed: bool,
    },

    /// We found checkboxes or  issue listing.
    Tracked {
        completed: u32,
        total: u32,
    },

    Error {
        message: String,
    },
}

#[derive(Serialize)]
struct TrackingIssueUpdate {
    pub author: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub url: String,
}

/// Identify how many sub-items have been completed.
/// These can be encoded in two different ways:
///
/// * Option A, the most common, is to have checkboxes in the issue. We just count the number that are checked.
/// * Option B is to include a metadata line called "Tracked issues" that lists a search query. We count the number of open vs closed issues in that query.
///
/// Returns a tuple (completed, total) with the number of completed items and the total number of items.
pub fn checkboxes(issue: &ExistingGithubIssue) -> Progress {
    match try_checkboxes(&issue) {
        Ok(pair) => pair,
        Err(e) => Progress::Error {
            message: e.to_string(),
        },
    }
}

fn try_checkboxes(issue: &ExistingGithubIssue) -> anyhow::Result<Progress> {
    let mut completed = 0;
    let mut total = 0;

    for line in issue.body.lines() {
        // Does this match TRACKED_ISSUES?
        if let Some(c) = re::TRACKED_ISSUES_QUERY.captures(line) {
            let repo = Repository::from_str(&c["repo"])?;
            let query = &c["query"];

            let CountIssues { open, closed } = count_issues_matching_search(&repo, query)?;
            completed += closed;
            total += open + closed;
            continue;
        }

        if let Some(c) = re::SEE_ALSO_QUERY.captures(line) {
            let issue_urls = c["issues"].split(&[',', ' ']).filter(|s| !s.is_empty());

            for issue_url in issue_urls {
                let c = match (
                    re::SEE_ALSO_ISSUE1.captures(issue_url),
                    re::SEE_ALSO_ISSUE2.captures(issue_url),
                ) {
                    (Some(c), _) => c,
                    (None, Some(c)) => c,
                    (None, None) => anyhow::bail!("invalid issue URL `{issue_url}`"),
                };
                let repository = Repository::new(&c["org"], &c["repo"]);
                let issue_number = c["issue"].parse::<u64>()?;
                let issue = fetch_issue(&repository, issue_number)?;
                match try_checkboxes(&issue)? {
                    Progress::Binary { is_closed } => {
                        if is_closed {
                            completed += 1;
                        }
                        total += 1;
                    }

                    Progress::Tracked {
                        completed: c,
                        total: t,
                    } => {
                        completed += c;
                        total += t;
                    }

                    Progress::Error { message } => {
                        anyhow::bail!("error parsing {repository}#{issue_number}: {message}")
                    }
                }
            }
        }

        if re::CHECKED_CHECKBOX.is_match(line) {
            total += 1;
            completed += 1;
        } else if re::CHECKBOX.is_match(line) {
            total += 1;
        }
    }

    if total == 0 && completed == 0 {
        Ok(Progress::Binary {
            is_closed: issue.state == ExistingIssueState::Closed,
        })
    } else {
        Ok(Progress::Tracked { completed, total })
    }
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

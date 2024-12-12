//! Generate JSON summarizing the tracking issues.
//!
//! This module contains types (e.g., [`TrackingIssues`]) that represent the
//! external API that is used by the website
//! and other tools to consume the tracking issue data. They are very similar
//! to the types in `gh` and so forth but because they represent
//! a versioned API, we copy them over here to insulate them from incidental changes.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    gh::{
        issue_id::Repository,
        issues::{
            count_issues_matching_search, fetch_issue, CountIssues, ExistingGithubIssue, ExistingIssueState,
        },
    },
    re,
};

#[derive(Serialize, Deserialize)]
pub struct TrackingIssues {
    pub repository: String,
    pub milestone: String,
    pub issues: Vec<TrackingIssue>,
}

#[derive(Serialize, Deserialize)]
pub struct TrackingIssue {
    /// Issue number on the repository
    pub number: u64,

    /// Title of the tracking issue
    pub title: String,

    /// True if this is a flagship goal
    pub flagship: bool,

    /// State of progress
    pub progress: Progress,

    /// Set of assigned people
    pub assignees: Vec<String>,

    /// Posts that we consider to be status updates, in chronological order
    pub updates: Vec<TrackingIssueUpdate>,

    /// Issue state
    pub state: ExistingIssueState,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Progress {
    /// We could not find any checkboxes or other details on the tracking issue.
    /// So all we have is "open" or "closed".
    Binary {
        is_closed: bool,
    },

    /// We found checkboxes or issue listing.
    Tracked {
        completed: u32,
        total: u32,
    },

    Error {
        message: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct TrackingIssueUpdate {
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

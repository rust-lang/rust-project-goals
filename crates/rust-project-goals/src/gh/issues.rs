use std::{collections::BTreeSet, process::Command, str::FromStr};

use anyhow::Context;
use chrono::NaiveDate;
use rust_project_goals_json::{GithubIssueState, Progress};
use serde::{Deserialize, Serialize};

use crate::{re, util::comma};

use super::{issue_id::Repository, labels::GhLabel, milestone::GhMilestone};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistingGithubIssue {
    pub number: u64,
    pub title: String,
    /// Just github username, no `@`
    pub assignees: BTreeSet<String>,
    pub comments: Vec<ExistingGithubComment>,
    pub body: String,
    pub state: GithubIssueState,
    pub labels: Vec<GhLabel>,
    pub milestone: Option<GhMilestone>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistingGithubComment {
    /// Just github username, no `@`
    pub author: String,
    pub body: String,
    pub created_at: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ExistingGithubIssueJson {
    title: String,
    number: u64,
    assignees: Vec<ExistingGithubAssigneeJson>,
    comments: Vec<ExistingGithubCommentJson>,
    body: String,
    state: GithubIssueState,
    labels: Vec<GhLabel>,
    milestone: Option<GhMilestone>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ExistingGithubAssigneeJson {
    login: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ExistingGithubCommentJson {
    body: String,
    author: ExistingGithubAuthorJson,
    #[serde(rename = "createdAt")]
    created_at: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ExistingGithubAuthorJson {
    login: String,
}

pub struct CountIssues {
    pub open: u32,
    pub closed: u32,
}

pub fn count_issues_matching_search(
    repository: &Repository,
    search: &str,
) -> anyhow::Result<CountIssues> {
    #[derive(Deserialize)]
    struct JustState {
        state: GithubIssueState,
    }

    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("list")
        .arg("-S")
        .arg(search)
        .arg("-s")
        .arg("all")
        .arg("--json")
        .arg("state")
        .output()?;

    let existing_issues: Vec<JustState> = serde_json::from_slice(&output.stdout)?;

    let mut count_issues = CountIssues { open: 0, closed: 0 };

    for issue in &existing_issues {
        match issue.state {
            GithubIssueState::Open => count_issues.open += 1,
            GithubIssueState::Closed => count_issues.closed += 1,
        }
    }

    Ok(count_issues)
}

pub fn fetch_issue(repository: &Repository, issue: u64) -> anyhow::Result<ExistingGithubIssue> {
    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("view")
        .arg(&format!("{issue}"))
        .arg("--json")
        .arg("title,assignees,number,comments,body,state,labels,milestone")
        .output()?;

    let e_i: ExistingGithubIssueJson = serde_json::from_slice(&output.stdout)?;

    Ok(ExistingGithubIssue::from(e_i))
}

pub fn list_issues_in_milestone(
    repository: &Repository,
    timeframe: &str,
) -> anyhow::Result<Vec<ExistingGithubIssue>> {
    list_issues(repository, &[("-m", timeframe)])
}

pub fn list_issues(
    repository: &Repository,
    filter: &[(&str, &str)],
) -> anyhow::Result<Vec<ExistingGithubIssue>> {
    let mut cmd = Command::new("gh");

    cmd.arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("list")
        .arg("-s")
        .arg("all")
        .arg("-L")
        .arg("5000");

    for (opt, val) in filter {
        cmd.arg(opt);
        cmd.arg(val);
    }

    let output = cmd
        .arg("--json")
        .arg("title,assignees,number,comments,body,state,labels,milestone")
        .output()
        .with_context(|| format!("running github cli tool `gh`"))?;

    let existing_issues: Vec<ExistingGithubIssueJson> = serde_json::from_slice(&output.stdout)?;

    Ok(existing_issues
        .into_iter()
        .map(|e_i| ExistingGithubIssue::from(e_i))
        .collect())
}

pub fn create_issue(
    repository: &Repository,
    body: &str,
    title: &str,
    labels: &[String],
    assignees: &BTreeSet<String>,
    milestone: &str,
) -> anyhow::Result<()> {
    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("create")
        .arg("-b")
        .arg(&body)
        .arg("-t")
        .arg(&title)
        .arg("-l")
        .arg(labels.join(","))
        .arg("-a")
        .arg(comma(&assignees))
        .arg("-m")
        .arg(&milestone)
        .output()?;

    if !output.status.success() {
        Err(anyhow::anyhow!(
            "failed to create issue `{}`: {}",
            title,
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(())
    }
}

pub fn change_milestone(
    repository: &Repository,
    number: u64,
    milestone: &str,
) -> anyhow::Result<()> {
    let mut command = Command::new("gh");
    command
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("edit")
        .arg(number.to_string())
        .arg("-m")
        .arg(milestone);

    let output = command.output()?;
    if !output.status.success() {
        Err(anyhow::anyhow!(
            "failed to change milestone `{}`: {}",
            number,
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(())
    }
}

pub fn create_comment(repository: &Repository, number: u64, body: &str) -> anyhow::Result<()> {
    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("comment")
        .arg(number.to_string())
        .arg("-b")
        .arg(body)
        .output()?;

    if !output.status.success() {
        Err(anyhow::anyhow!(
            "failed to leave comment on issue `{}`: {}",
            number,
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(())
    }
}

pub fn sync_assignees(
    repository: &Repository,
    number: u64,
    remove_owners: &BTreeSet<String>,
    add_owners: &BTreeSet<String>,
) -> anyhow::Result<()> {
    let mut command = Command::new("gh");
    command
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("edit")
        .arg(number.to_string());

    if !remove_owners.is_empty() {
        command.arg("--remove-assignee").arg(comma(&remove_owners));
    }

    if !add_owners.is_empty() {
        command.arg("--add-assignee").arg(comma(&add_owners));
    }

    let output = command.output()?;
    if !output.status.success() {
        Err(anyhow::anyhow!(
            "failed to sync issue `{}`: {}",
            number,
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(())
    }
}

pub const FLAGSHIP_LABEL: &str = "Flagship Goal";

pub const LOCK_TEXT: &str = "This issue is intended for status updates only.\n\nFor general questions or comments, please contact the owner(s) directly.";

impl ExistingGithubIssue {
    /// We use the presence of a "lock comment" as a signal that we successfully locked the issue.
    /// The github CLI doesn't let you query that directly.
    pub fn was_locked(&self) -> bool {
        self.comments.iter().any(|c| c.body.trim() == LOCK_TEXT)
    }

    /// True if we have a label with the given name.
    pub fn has_label(&self, name: &str) -> bool {
        self.labels.iter().any(|label| label.name == name)
    }

    /// True if the issue has the label for a flagship goal.
    pub fn has_flagship_label(&self) -> bool {
        self.has_label(FLAGSHIP_LABEL)
    }
}

pub fn lock_issue(repository: &Repository, number: u64) -> anyhow::Result<()> {
    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("lock")
        .arg(number.to_string())
        .output()?;

    if !output.status.success() {
        if !output.stderr.starts_with(b"already locked") {
            return Err(anyhow::anyhow!(
                "failed to lock issue `{}`: {}",
                number,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    Ok(())
}

impl ExistingGithubComment {
    /// True if this is one of the special comments that we put on issues.
    pub fn is_automated_comment(&self) -> bool {
        self.body.trim() == LOCK_TEXT
    }

    pub fn created_at_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.created_at, "%Y-%m-%dT%H:%M:%SZ")
            .expect("failed to parse date")
    }
}

impl From<ExistingGithubIssueJson> for ExistingGithubIssue {
    fn from(e_i: ExistingGithubIssueJson) -> Self {
        ExistingGithubIssue {
            number: e_i.number,
            title: e_i.title,
            assignees: e_i.assignees.into_iter().map(|a| a.login).collect(),
            comments: e_i
                .comments
                .into_iter()
                .map(|c| ExistingGithubComment {
                    author: format!("@{}", c.author.login),
                    body: c.body,
                    url: c.url,
                    created_at: c.created_at,
                })
                .collect(),
            body: e_i.body,
            state: e_i.state,
            labels: e_i.labels,
            milestone: e_i.milestone,
        }
    }
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
            is_closed: issue.state == GithubIssueState::Closed,
        })
    } else {
        Ok(Progress::Tracked { completed, total })
    }
}

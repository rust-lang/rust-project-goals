use std::{
    collections::{BTreeMap, BTreeSet},
    process::Command,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::util::comma;

use super::{issue_id::Repository, labels::GhLabel};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistingGithubIssue {
    pub number: u64,
    /// Just github username, no `@`
    pub assignees: BTreeSet<String>,
    pub comments: Vec<ExistingGithubComment>,
    pub body: String,
    pub state: ExistingIssueState,
    pub labels: Vec<GhLabel>,
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
    state: ExistingIssueState,
    labels: Vec<GhLabel>,
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum ExistingIssueState {
    Open,
    Closed,
}

impl std::fmt::Display for ExistingIssueState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExistingIssueState::Open => write!(f, "open"),
            ExistingIssueState::Closed => write!(f, "closed"),
        }
    }
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
        state: ExistingIssueState,
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
            ExistingIssueState::Open => count_issues.open += 1,
            ExistingIssueState::Closed => count_issues.closed += 1,
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
        .arg("title,assignees,number,comments,body,state,labels")
        .output()?;

    let e_i: ExistingGithubIssueJson = serde_json::from_slice(&output.stdout)?;

    Ok(ExistingGithubIssue::from(e_i))
}

pub fn list_issue_titles_in_milestone(
    repository: &Repository,
    timeframe: &str,
) -> anyhow::Result<BTreeMap<String, ExistingGithubIssue>> {
    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("list")
        .arg("-m")
        .arg(timeframe)
        .arg("-s")
        .arg("all")
        .arg("--json")
        .arg("title,assignees,number,comments,body,state,labels")
        .output()?;

    let existing_issues: Vec<ExistingGithubIssueJson> = serde_json::from_slice(&output.stdout)?;

    Ok(existing_issues
        .into_iter()
        .map(|e_i| (e_i.title.clone(), ExistingGithubIssue::from(e_i)))
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

const LOCK_TEXT: &str = "This issue is intended for status updates only.\n\nFor general questions or comments, please contact the owner(s) directly.";

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

    // Leave a comment explaining what is going on.
    let output = Command::new("gh")
        .arg("-R")
        .arg(&repository.to_string())
        .arg("issue")
        .arg("comment")
        .arg(number.to_string())
        .arg("-b")
        .arg(LOCK_TEXT)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "failed to leave lock comment `{}`: {}",
            number,
            String::from_utf8_lossy(&output.stderr)
        ));
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
        }
    }
}

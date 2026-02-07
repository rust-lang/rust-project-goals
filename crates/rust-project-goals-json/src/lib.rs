//! This module contains types (e.g., [`TrackingIssues`]) that represent the
//! external API that is used by the website
//! and other tools to consume the tracking issue data. They are very similar
//! to the types in `gh` and so forth but because they represent
//! a versioned API, we copy them over here to insulate them from incidental changes.

use serde::{Deserialize, Serialize};

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

    /// True if this is a roadmap goal
    pub roadmap: bool,

    /// State of progress
    pub progress: Progress,

    /// Set of assigned people
    pub assignees: Vec<String>,

    /// Posts that we consider to be status updates, in chronological order
    pub updates: Vec<TrackingIssueUpdate>,

    /// Issue state
    pub state: GithubIssueState,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum GithubIssueState {
    Open,
    Closed,
}

impl std::fmt::Display for GithubIssueState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubIssueState::Open => write!(f, "open"),
            GithubIssueState::Closed => write!(f, "closed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

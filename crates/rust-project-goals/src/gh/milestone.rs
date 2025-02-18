use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GhMilestone {
    pub number: u64,
    pub title: String,
    pub description: String,
    #[serde(rename = "dueOn")]
    pub due_on: Option<String>,
}

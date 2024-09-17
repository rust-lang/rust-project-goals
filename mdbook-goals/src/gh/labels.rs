use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GhLabel {
    pub name: String,
    pub color: String,
}

pub fn list_labels(repository: &str) -> anyhow::Result<Vec<GhLabel>> {
    let output = Command::new("gh")
        .arg("-R")
        .arg(repository)
        .arg("label")
        .arg("list")
        .arg("--json")
        .arg("name,color")
        .output()?;

    let labels: Vec<GhLabel> = serde_json::from_slice(&output.stdout)?;

    Ok(labels)
}

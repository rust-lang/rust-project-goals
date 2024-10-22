use std::process::Command;

use serde::{Deserialize, Serialize};

use super::issue_id::Repository;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GhLabel {
    pub name: String,
    pub color: String,
}

impl GhLabel {
    pub fn list(repository: &Repository) -> anyhow::Result<Vec<GhLabel>> {
        let output = Command::new("gh")
            .arg("-R")
            .arg(&repository.to_string())
            .arg("label")
            .arg("list")
            .arg("--json")
            .arg("name,color")
            .output()?;

        let labels: Vec<GhLabel> = serde_json::from_slice(&output.stdout)?;

        Ok(labels)
    }

    pub fn create(&self, repository: &Repository) -> anyhow::Result<()> {
        let output = Command::new("gh")
            .arg("-R")
            .arg(&repository.to_string())
            .arg("label")
            .arg("create")
            .arg(&self.name)
            .arg("--color")
            .arg(&self.color)
            .arg("--force")
            .output()?;

        if !output.status.success() {
            Err(anyhow::anyhow!(
                "failed to create label `{}`: {}",
                self.name,
                String::from_utf8_lossy(&output.stderr)
            ))
        } else {
            Ok(())
        }
    }
}

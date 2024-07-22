use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    goal::{self, GoalDocument},
    team::TeamName,
};

fn validate_path(path: &Path) -> anyhow::Result<String> {
    if !path.is_dir() {
        return Err(anyhow::anyhow!(
            "RFC path should be a directory like src/2024h2"
        ));
    };

    if path.is_absolute() {
        return Err(anyhow::anyhow!("RFC path should be relative"));
    }

    let timeframe = path
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid path `{}`", path.display()))?;

    Ok(timeframe.to_string())
}

pub fn generate_comment(path: &Path) -> anyhow::Result<()> {
    let _ = validate_path(path)?;
    let goal_documents = goal::goals_in_dir(path)?;
    let teams_with_asks = teams_with_asks(&goal_documents);

    for team_name in teams_with_asks {
        let team_data = team_name.data();

        println!("\n## {}\n", team_data.name);

        let (leads, members): (Vec<_>, Vec<_>) = team_data.members.iter().partition(|m| m.is_lead);

        for lead in leads {
            println!("* [ ] @{} (required, lead)", lead.github);
        }

        for member in members {
            println!("* [ ] @{} (optional)", member.github);
        }
    }

    Ok(())
}

pub fn generate_rfc(path: &Path) -> anyhow::Result<()> {
    let timeframe = &validate_path(path)?;

    // run mdbook build
    Command::new("mdbook").arg("build").status()?;

    // find the markdown output
    let generated_path = PathBuf::from("book/markdown")
        .join(timeframe)
        .join("index.md");
    if !generated_path.exists() {
        return Err(anyhow::anyhow!(
            "no markdown generated at {}",
            generated_path.display()
        ));
    }

    let generated_text = std::fs::read_to_string(&generated_path).with_context(|| {
        format!(
            "reading generated markdown from `{}`",
            generated_path.display()
        )
    })?;

    let regex = Regex::new(r"\((.*).md(#[^)]*)?\)").unwrap();

    let result = regex.replace_all(
        &generated_text,
        format!("(https://rust-lang.github.io/rust-project-goals/{timeframe}/$1.html$2)"),
    );

    println!("{result}");

    Ok(())
}

pub fn generate_issues(repository: &str, path: &Path, dry_run: bool) -> anyhow::Result<()> {
    let _ = validate_path(path)?;

    let goal_documents = goal::goals_in_dir(path)?;
    let teams_with_asks = teams_with_asks(&goal_documents);
    let mut actions = initialize_labels(repository, &teams_with_asks)?;
    actions.extend(initialize_issues(repository, &goal_documents)?);

    if actions.is_empty() {
        eprintln!("No actions to be executed.");
        return Ok(());
    }

    eprintln!("Actions to be executed:");
    for action in &actions {
        eprintln!("* {action}");
    }

    if !dry_run {
        for action in actions {
            action.execute(repository)?;
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GithubIssue {
    pub title: String,
    pub owners: Vec<String>,
    pub body: String,
    pub teams: BTreeSet<&'static TeamName>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GithubAction {
    CreateLabel { label: GhLabel },
    CreateIssue { issue: GithubIssue },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct GhLabel {
    name: String,
    color: String,
}

fn list_labels(repository: &str) -> anyhow::Result<Vec<GhLabel>> {
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

/// Initializes the required `T-<team>` labels on the repository.
/// Warns if the labels are found with wrong color.
fn initialize_labels(
    repository: &str,
    teams_with_asks: &BTreeSet<&TeamName>,
) -> anyhow::Result<BTreeSet<GithubAction>> {
    const TEAM_LABEL_COLOR: &str = "bfd4f2";

    let mut desired_labels: BTreeSet<_> = teams_with_asks
        .iter()
        .map(|team| {
            let label_name = team.gh_label();

            GhLabel {
                name: label_name,
                color: TEAM_LABEL_COLOR.to_string(),
            }
        })
        .collect();

    for existing_label in list_labels(repository)? {
        desired_labels.remove(&existing_label);
    }

    Ok(desired_labels
        .into_iter()
        .map(|label| GithubAction::CreateLabel { label })
        .collect())
}

/// Initializes the required `T-<team>` labels on the repository.
/// Warns if the labels are found with wrong color.
fn initialize_issues(
    repository: &str,
    document: &[GoalDocument],
) -> anyhow::Result<BTreeSet<GithubAction>> {
    // let issues: Vec<_> = goal_documents
    //     .iter()
    //     .map(|goal_document| {
    //         let title = format!("Goal: {}", goal_document.title);
    //         let owners = goal_document.metadata.owner_usernames();
    //         let body = goal_document.description.clone();
    //         let teams = goal_document
    //             .team_asks
    //             .iter()
    //             .flat_map(|ask| &ask.teams)
    //             .copied()
    //             .collect::<BTreeSet<&TeamName>>();

    //         GithubIssue {
    //             title,
    //             owners,
    //             body,
    //             teams,
    //         }
    //     })
    //     .collect();

    Ok(None.into_iter().collect())
}

fn teams_with_asks(goal_documents: &[GoalDocument]) -> BTreeSet<&'static TeamName> {
    goal_documents
        .iter()
        .flat_map(|g| &g.team_asks)
        .flat_map(|ask| &ask.teams)
        .copied()
        .collect()
}

impl Display for GithubAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubAction::CreateLabel {
                label: GhLabel { name, color },
            } => {
                write!(f, "create label `{}` with color `{}`", name, color)
            }
            GithubAction::CreateIssue { issue } => {
                write!(f, "create issue `{}`", issue.title)
            }
        }
    }
}

impl GithubAction {
    pub fn execute(self, repository: &str) -> anyhow::Result<()> {
        match self {
            GithubAction::CreateLabel {
                label: GhLabel { name, color },
            } => {
                let output = Command::new("gh")
                    .arg("-R")
                    .arg(repository)
                    .arg("label")
                    .arg("create")
                    .arg(&name)
                    .arg("--color")
                    .arg(&color)
                    .arg("--force")
                    .output()?;

                if !output.status.success() {
                    Err(anyhow::anyhow!(
                        "failed to create label `{}`: {}",
                        name,
                        String::from_utf8_lossy(&output.stderr)
                    ))
                } else {
                    Ok(())
                }
            }

            GithubAction::CreateIssue { issue } => todo!(),
        }
    }
}

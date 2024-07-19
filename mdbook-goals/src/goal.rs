use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;
use std::{collections::BTreeSet, path::PathBuf};

use anyhow::Context;
use regex::Regex;

use crate::re::USERNAME;
use crate::team::{self, TeamName};
use crate::util::{commas, markdown_files};
use crate::{
    markwaydown::{self, Section, Table},
    util::{self, ARROW},
};

/// Data parsed from a goal file in the expected format
pub struct GoalDocument {
    /// Path relative to the current directory (`book.toml`)
    pub path: PathBuf,

    /// Path relative to the directory of goals this goal is a part of,
    /// and hence suitable for links in other markdown files.
    pub link_path: Arc<PathBuf>,

    /// Metadata loaded from the header in the goal
    pub metadata: Metadata,

    /// List of team asks extracted from the goal
    pub team_asks: Vec<TeamAsk>,
}

/// Metadata loaded from the goal header
#[derive(Debug)]
pub struct Metadata {
    #[allow(unused)]
    pub title: String,
    pub short_title: String,
    pub owners: String,
    pub status: Status,
}

/// Identifies a particular ask for a set of Rust teams
#[derive(Debug)]
pub struct TeamAsk {
    /// Path to the markdown file containing this ask (appropriate for a link)
    pub link_path: Arc<PathBuf>,

    /// Title of the subgoal (or goal, if there are no subgoals)
    pub subgoal: String,

    /// What the team is being asked for (e.g., RFC decision)
    pub heading: String,

    /// Name(s) of the teams being asked to do the thing
    pub teams: Vec<&'static TeamName>,

    /// Owners of the subgoal or goal
    pub owners: String,

    /// Any notes
    pub notes: String,
}

/// Load all the goals from a given directory
pub fn goals_in_dir(directory_path: &Path) -> anyhow::Result<Vec<GoalDocument>> {
    let mut goal_documents = vec![];
    for (path, link_path) in markdown_files(&directory_path)? {
        if let Some(goal_document) = GoalDocument::load(&path, &link_path)
            .with_context(|| format!("loading goal from `{}`", path.display()))?
        {
            goal_documents.push(goal_document);
        }
    }
    Ok(goal_documents)
}

impl GoalDocument {
    fn load(path: &Path, link_path: &Path) -> anyhow::Result<Option<Self>> {
        let sections = markwaydown::parse(path)?;

        let Some(metadata) = extract_metadata(&sections)? else {
            return Ok(None);
        };

        let link_path = Arc::new(link_path.to_path_buf());
        let team_asks = match metadata.status {
            Status::Flagship | Status::Proposed | Status::Orphaned => {
                extract_team_asks(&link_path, &metadata, &sections)?
            }
            Status::NotAccepted => vec![],
        };

        if metadata.status != Status::NotAccepted && team_asks.is_empty() {
            anyhow::bail!("no team asks found in goal file `{}`", path.display());
        }

        Ok(Some(GoalDocument {
            path: path.to_path_buf(),
            link_path,
            metadata,
            team_asks,
        }))
    }
}

/// Format a set of team asks into a table, with asks separated by team and grouped by kind.
pub fn format_team_asks(asks_of_any_team: &[&TeamAsk]) -> anyhow::Result<String> {
    let mut output = String::new();

    let all_teams: BTreeSet<&TeamName> = asks_of_any_team
        .iter()
        .flat_map(|a| &a.teams)
        .copied()
        .collect();

    for team_name in all_teams {
        let asks_of_this_team: Vec<_> = asks_of_any_team
            .iter()
            .filter(|a| a.teams.contains(&team_name))
            .collect();

        let team_data = team_name.data();
        write!(output, "\n### {} team\n", team_data.name)?;

        let subgoals: BTreeSet<&String> = asks_of_this_team.iter().map(|a| &a.subgoal).collect();

        let mut table = vec![vec![
            "Goal".to_string(),
            "Owner".to_string(),
            "Notes".to_string(),
        ]];

        for subgoal in subgoals {
            table.push(vec![
                format!("*{}*", subgoal),
                "".to_string(),
                "".to_string(),
            ]);

            for ask in asks_of_this_team.iter().filter(|a| a.subgoal == *subgoal) {
                table.push(vec![
                    format!(
                        "{} [{}]({}#ownership-and-team-asks)",
                        ARROW,
                        ask.heading,
                        ask.link_path.display()
                    ),
                    ask.owners.to_string(),
                    ask.notes.to_string(),
                ]);
            }
        }

        write!(output, "{}", util::format_table(&table))?;
    }

    Ok(output)
}

pub fn format_goal_table(goals: &[&GoalDocument]) -> anyhow::Result<String> {
    let mut table = vec![vec![
        "Goal".to_string(),
        "Owner".to_string(),
        "Team".to_string(),
    ]];

    for goal in goals {
        let teams: BTreeSet<&TeamName> = goal
            .team_asks
            .iter()
            .flat_map(|ask| &ask.teams)
            .copied()
            .collect();
        let teams: Vec<&TeamName> = teams.into_iter().collect();
        table.push(vec![
            format!("[{}]({})", goal.metadata.title, goal.link_path.display()),
            goal.metadata.owners.clone(),
            commas(&teams),
        ]);
    }

    Ok(util::format_table(&table))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Flagship,
    Proposed,
    Orphaned,
    NotAccepted,
}

impl TryFrom<&str> for Status {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let status_values = &[
            ("Flagship", Status::Flagship),
            ("Proposed", Status::Proposed),
            ("Orphaned", Status::Orphaned),
            ("Not accepted", Status::NotAccepted),
        ];

        status_values
            .iter()
            .find(|pair| value == pair.0)
            .map(|pair| pair.1)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "unrecognized status `{}`, expected one of: {}",
                    value,
                    commas(status_values.iter().map(|pair| pair.0))
                )
            })
    }
}

fn extract_metadata(sections: &[Section]) -> anyhow::Result<Option<Metadata>> {
    let Some(first_section) = sections.first() else {
        anyhow::bail!("no markdown sections found in input")
    };

    if first_section.title.is_empty() {
        anyhow::bail!("first section has no title");
    }

    let title = &first_section.title;

    let Some(first_table) = first_section.tables.first() else {
        return Ok(None);
    };

    expect_headers(first_table, &["Metadata", ""])?;

    let short_title_row = first_table.rows.iter().find(|row| row[0] == "Short title");

    let Some(owners_row) = first_table
        .rows
        .iter()
        .find(|row| row[0] == "Owner" || row[0] == "Owner(s)" || row[0] == "Owners")
    else {
        anyhow::bail!("metadata table has no `Owner(s)` row")
    };

    let Some(status_row) = first_table.rows.iter().find(|row| row[0] == "Status") else {
        anyhow::bail!("metadata table has no `Status` row")
    };

    let status = Status::try_from(status_row[1].as_str())?;

    Ok(Some(Metadata {
        title: title.to_string(),
        short_title: if let Some(row) = short_title_row {
            row[1].to_string()
        } else {
            title.to_string()
        },
        owners: owners_row[1].to_string(),
        status,
    }))
}

fn extract_team_asks<'i>(
    link_path: &Arc<PathBuf>,
    metadata: &Metadata,
    sections: &[Section],
) -> anyhow::Result<Vec<TeamAsk>> {
    let Some(ownership_section) = sections
        .iter()
        .find(|section| section.title == "Ownership and team asks")
    else {
        anyhow::bail!("no `Ownership and team asks` section found")
    };

    let Some(table) = ownership_section.tables.first() else {
        anyhow::bail!(
            "on line {}, no table found in `Ownership and team asks` section",
            ownership_section.line_num
        )
    };

    expect_headers(table, &["Subgoal", "Owner(s) or team(s)", "Notes"])?;

    let mut heading = "";
    let mut heading_owners: &str = &metadata.owners[..];

    let mut tasks = vec![];
    for row in &table.rows {
        let subgoal;
        let owners;
        if row[0].starts_with(ARROW) {
            // e.g., "↳ stabilization" is a subtask of the metagoal
            subgoal = row[0][ARROW.len()..].trim();
            owners = heading_owners;
        } else {
            // remember the last heading
            heading = &row[0];
            heading_owners = if row[1].is_empty() {
                &metadata.owners[..]
            } else {
                &row[1]
            };

            subgoal = heading;
            owners = &metadata.owners;
        };

        if !row[1].contains("![Team]") {
            continue;
        }

        let mut teams = vec![];
        for team_name in extract_team_names(&row[1]) {
            let Some(team) = team::get_team_name(&team_name)? else {
                anyhow::bail!(
                    "no Rust team named `{}` found (valid names are {})",
                    team_name,
                    commas(team::get_team_names()?),
                );
            };

            teams.push(team);
        }

        if teams.is_empty() {
            anyhow::bail!("team ask for \"{subgoal}\" does not list any teams");
        }

        tasks.push(TeamAsk {
            link_path: link_path.clone(),
            heading: if subgoal == heading {
                metadata.short_title.to_string()
            } else {
                heading.to_string()
            },
            subgoal: subgoal.to_string(),
            teams,
            owners: owners.to_string(),
            notes: row[2].to_string(),
        });
    }

    Ok(tasks)
}

fn expect_headers(table: &Table, expected: &[&str]) -> anyhow::Result<()> {
    if table.header != expected {
        anyhow::bail!(
            "on line {}, unexpected table header, expected `{:?}`, found `{:?}`",
            table.line_num,
            expected,
            table.header
        );
    }

    Ok(())
}

fn extract_team_names(s: &str) -> Vec<String> {
    extract_identifiers(s)
        .into_iter()
        .filter(|&s| s != "Team")
        .map(|s| s.to_string())
        .collect()
}

fn extract_identifiers(s: &str) -> Vec<&str> {
    let regex = Regex::new("[-.A-Za-z]+").unwrap();
    regex.find_iter(s).map(|m| m.as_str()).collect()
}

impl Metadata {
    /// Extracts the `@abc` usernames found in the owner listing.
    pub fn owner_usernames(&self) -> Vec<&str> {
        self.owners
            .split(char::is_whitespace)
            .filter_map(|owner| USERNAME.captures(owner))
            .map(|captures| captures.get(0).unwrap().as_str())
            .collect()
    }
}

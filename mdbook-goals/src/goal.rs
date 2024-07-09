use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

use regex::Regex;

use crate::{
    markwaydown::{self, Section, Table},
    util::{self, ARROW},
};

/// Process the input file `input` and return a list of team asks.
/// Ignores goals that are marked as "not accepted".
///
/// # Parameters
///
/// * `input`, path on disk
/// * `link_path`, path to insert into any links in the output
pub fn team_asks_in_input<'i>(
    input: &'i Path,
    link_path: &'i Path,
) -> anyhow::Result<Vec<TeamAsk<'i>>> {
    let sections = markwaydown::parse(input)?;

    let Some(metadata) = extract_metadata(&sections)? else {
        return Ok(vec![]);
    };

    match metadata.status {
        Status::Flagship | Status::Proposed => extract_team_asks(link_path, &metadata, &sections),
        Status::NotAccepted => Ok(vec![]),
    }
}

pub fn format_team_asks(asks_of_any_team: &[TeamAsk]) -> anyhow::Result<String> {
    let mut output = String::new();

    let all_teams: BTreeSet<&String> = asks_of_any_team.iter().flat_map(|a| &a.teams).collect();

    for team in all_teams {
        let asks_of_this_team: Vec<&TeamAsk> = asks_of_any_team
            .iter()
            .filter(|a| a.teams.contains(team))
            .collect();

        if team != "LC" {
            write!(output, "\n### {} team\n", team)?;
        } else {
            write!(output, "\n### Leadership Council\n")?;
        }

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

#[derive(Debug)]
struct Metadata<'a> {
    #[allow(unused)]
    title: &'a str,
    short_title: &'a str,
    owners: &'a str,
    status: Status,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Flagship,
    Proposed,
    NotAccepted,
}

fn extract_metadata(sections: &[Section]) -> anyhow::Result<Option<Metadata<'_>>> {
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

    let status_values = &[
        ("Flagship", Status::Flagship),
        ("Proposed", Status::Proposed),
        ("Not accepted", Status::NotAccepted),
    ];

    let Some(status) = status_values
        .iter()
        .find(|(s, _)| status_row[1] == *s)
        .map(|s| s.1)
    else {
        anyhow::bail!(
            "unrecognized status `{}`, expected one of: {}",
            status_row[1],
            status_values
                .iter()
                .map(|s| s.0)
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    Ok(Some(Metadata {
        title,
        short_title: if let Some(row) = short_title_row {
            &row[1]
        } else {
            title
        },
        owners: &owners_row[1],
        status,
    }))
}

#[derive(Debug)]
pub struct TeamAsk<'i> {
    link_path: &'i Path,
    subgoal: String,
    heading: String,
    teams: Vec<String>,
    owners: String,
    notes: String,
}

fn extract_team_asks<'i>(
    link_path: &'i Path,
    metadata: &Metadata<'_>,
    sections: &[Section],
) -> anyhow::Result<Vec<TeamAsk<'i>>> {
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
    let mut owners: &str = metadata.owners;

    let mut tasks = vec![];
    for row in &table.rows {
        let subgoal;
        if row[0].starts_with(ARROW) {
            // e.g., "↳ stabilization" is a subtask of the metagoal
            subgoal = row[0][ARROW.len()..].trim().to_string();
        } else {
            // remember the last heading
            heading = &row[0];
            subgoal = heading.to_string();
        };

        if !row[1].contains("![Team]") {
            if !row[1].is_empty() {
                owners = &row[1];
            }

            continue;
        }

        let teams = extract_teams(&row[1]);

        tasks.push(TeamAsk {
            link_path,
            heading: if subgoal == heading {
                metadata.short_title.to_string()
            } else {
                heading.to_string()
            },
            subgoal,
            teams,
            owners: if owners == "Owner" {
                metadata.owners.to_string()
            } else {
                owners.to_string()
            },
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

fn extract_teams(s: &str) -> Vec<String> {
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

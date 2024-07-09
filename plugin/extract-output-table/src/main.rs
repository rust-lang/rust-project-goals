use anyhow::Context;
use markwaydown::{Section, Table};
use regex::Regex;
use std::{
    collections::BTreeSet,
    fmt::Write,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

mod markwaydown;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(long)]
    status: Option<String>,
    inputs: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let mut all_team_asks = vec![];
    for input in &opt.inputs {
        all_team_asks.extend(
            process_input(input, &opt.status)
                .with_context(|| format!("parsing `{}` as markdown", input.display()))?,
        );
    }

    format_team_asks(&all_team_asks)
}

fn process_input<'i>(
    input: &'i Path,
    status_filter: &Option<String>,
) -> anyhow::Result<Vec<TeamAsk<'i>>> {
    let sections = markwaydown::parse(input)?;

    let metadata = extract_metadata(&sections)?;

    if let Some(s) = status_filter {
        if metadata.status != s {
            return Ok(vec![]);
        }
    }

    extract_team_asks(input, &metadata, &sections)
}

fn format_team_asks(asks_of_any_team: &[TeamAsk]) -> anyhow::Result<()> {
    let all_teams: BTreeSet<&String> = asks_of_any_team.iter().flat_map(|a| &a.teams).collect();

    for team in all_teams {
        let asks_of_this_team: Vec<&TeamAsk> = asks_of_any_team
            .iter()
            .filter(|a| a.teams.contains(team))
            .collect();

        println!("\n### {} team\n", team);

        let subgoals: BTreeSet<&String> = asks_of_this_team.iter().map(|a| &a.subgoal).collect();

        let mut table = vec![
            vec!["Goal".to_string(), "Owner".to_string()],
            vec!["---".to_string(), "---".to_string()],
        ];

        for subgoal in subgoals {
            table.push(vec![format!("*{}*", subgoal), "".to_string()]);

            for ask in asks_of_this_team.iter().filter(|a| a.subgoal == *subgoal) {
                table.push(vec![
                    format!("[{}]({})", ask.heading, ask.input.display()),
                    ask.owners.to_string(),
                ]);
            }
        }

        println!("{}", format_table(&table));
    }

    Ok(())
}

#[derive(Debug)]
struct Metadata<'a> {
    #[allow(unused)]
    title: &'a str,
    short_title: &'a str,
    owners: &'a str,
    status: &'a str,
}

fn extract_metadata(sections: &[Section]) -> anyhow::Result<Metadata<'_>> {
    let Some(first_section) = sections.first() else {
        anyhow::bail!("no markdown sections found in input")
    };

    if first_section.title.is_empty() {
        anyhow::bail!("first section has no title");
    }

    let title = &first_section.title;

    let Some(first_table) = first_section.tables.first() else {
        anyhow::bail!("no metadata table found in first section")
    };

    if first_table.header.len() < 2 {
        anyhow::bail!("metadata table has too few columns, expected 2");
    }

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

    Ok(Metadata {
        title,
        short_title: if let Some(row) = short_title_row {
            &row[1]
        } else {
            title
        },
        owners: &owners_row[1],
        status: &status_row[1],
    })
}

#[derive(Debug)]
struct TeamAsk<'i> {
    input: &'i Path,
    subgoal: String,
    heading: String,
    teams: Vec<String>,
    owners: String,
}

fn extract_team_asks<'i>(
    input: &'i Path,
    metadata: &Metadata<'_>,
    sections: &[Section],
) -> anyhow::Result<Vec<TeamAsk<'i>>> {
    let Some(ownership_section) = sections
        .iter()
        .find(|section| section.title == "Ownership and other resources")
    else {
        anyhow::bail!("no `Ownership and other resources` section found")
    };

    let Some(table) = ownership_section.tables.first() else {
        anyhow::bail!(
            "on line {}, no table found in `Ownership and other resources` section",
            ownership_section.line_num
        )
    };

    expect_headers(table, &["Subgoal", "Owner(s) or team(s)", "Notes"])?;

    let mut heading = "";
    let mut owners: &str = metadata.owners;

    let mut tasks = vec![];
    for row in &table.rows {
        const ARROW: &str = "↳";

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
            input,
            heading: if subgoal == heading {
                metadata.short_title.to_string()
            } else {
                heading.to_string()
            },
            subgoal,
            teams,
            owners: owners.to_string(),
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
    let regex = Regex::new("[-A-Za-z]+").unwrap();
    regex.find_iter(s).map(|m| m.as_str()).collect()
}

fn format_table(rows: &[Vec<String>]) -> String {
    let mut output = String::new();

    let columns = rows[0].len();
    let mut widths = vec![0; columns];

    for columns in rows {
        for (text, col) in columns.iter().zip(0..) {
            widths[col] = widths[col].max(text.len());
        }
    }

    for columns in rows {
        for (text, col) in columns.iter().zip(0..) {
            output.push('|');

            write!(output, " {text:<width$} ", text = text, width = widths[col]).unwrap();
        }

        output.push('|');
        output.push('\n');
    }

    output
}

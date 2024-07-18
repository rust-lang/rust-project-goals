use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use regex::Regex;

use crate::{goal, team::TeamName};

pub fn generate_comment(path: &Path) -> anyhow::Result<()> {
    let goal_documents = goal::goals_in_dir(path)?;
    let teams_with_asks: BTreeSet<&TeamName> = goal_documents
        .iter()
        .flat_map(|g| &g.team_asks)
        .flat_map(|ask| &ask.teams)
        .copied()
        .collect();

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

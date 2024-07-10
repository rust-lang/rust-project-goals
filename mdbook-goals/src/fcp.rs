use std::{collections::BTreeSet, path::Path};

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

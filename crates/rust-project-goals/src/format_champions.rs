use std::collections::{BTreeMap, BTreeSet};

use spanned::{Result, Spanned};

use crate::{goal::GoalDocument, util};

/// Format a champions table showing each champion and their goals.
pub fn format_champions(goals: &[&GoalDocument]) -> Result<String> {
    use std::fmt::Write;

    let mut output = String::new();

    // Collect champions and their goals
    let mut champion_goals: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for goal in goals {
        for champion in goal.metadata.champions.values() {
            let champion_name = champion.content.clone();
            let goal_link = format!("° [{}]({})", goal.metadata.title.content, goal.link_path.display());
            
            champion_goals
                .entry(champion_name)
                .or_default()
                .insert(goal_link);
        }
    }

    if champion_goals.is_empty() {
        return Ok("No champions found.".to_string());
    }

    // Create the table
    let table = {
        let headings = vec![
            Spanned::here("Champion".to_string()),
            Spanned::here("#".to_string()),
            Spanned::here("Goals".to_string()),
        ];

        let rows = champion_goals.into_iter().map(|(champion, goals)| {
            let goals_vec: Vec<String> = goals.into_iter().collect();
            vec![
                Spanned::here(champion),
                Spanned::here(goals_vec.len().to_string()),
                Spanned::here(goals_vec.join("<br>")),
            ]
        });

        std::iter::once(headings).chain(rows).collect::<Vec<_>>()
    };

    write!(output, "{}", util::format_table(&table))?;

    Ok(output)
}

use std::collections::BTreeSet;

use indexmap::IndexMap;

use crate::{config::Configuration, goal::TeamAsk, team::TeamName, util};

/// Format a set of team asks into a table, with asks separated by team and grouped by kind.
///
/// output looks like
///
/// ```ignore
/// | Goal      | [DMS](#discussion-and-moral-support) | [SR](#standard reviews) |
/// | :---      | :--                                  | :--                     |
/// | Foo       | ✅                                   | ✅ (notes)               |
/// | Bar (Baz) | ✅                                   | ✅ (\*1)                 |
///
/// \*1: ... longer notes that would not fit ...
/// ```
pub fn format_team_asks(asks_of_any_team: &[&TeamAsk]) -> anyhow::Result<String> {
    use std::fmt::Write;

    const CHECK: &str = "✅";

    /// Arbitrary: max length of text before we insert a footnote
    const FOOTNOTE_LEN: usize = 22;

    let mut output = String::new();

    let all_teams: BTreeSet<&TeamName> = asks_of_any_team
        .iter()
        .flat_map(|a| &a.teams)
        .copied()
        .collect();

    // The set of configured team asks
    let config = Configuration::get();

    for team_name in all_teams {
        let asks_of_this_team: Vec<_> = asks_of_any_team
            .iter()
            .filter(|a| a.teams.contains(&team_name))
            .collect();

        let team_data = team_name.data();
        write!(output, "\n### {} team\n", team_data.name)?;

        // We will accumulate footnotes when we encounter comments that are too long.
        let mut footnotes = vec![];

        // These are things like "discussion and moral support". They are extracted from
        // the configuration. We prune out the ones that do not appear in the asks for a particular team.
        let ask_headings = config
            .team_asks
            .keys()
            .filter(|&ask_kind| {
                asks_of_this_team
                    .iter()
                    .any(|a| &a.ask_description == ask_kind)
            })
            .collect::<Vec<_>>();
        let empty_row = || {
            (0..ask_headings.len())
                .map(|_| "".to_string())
                .collect::<Vec<_>>()
        };

        // Collect the asks by goal. The `rows` map goes from goal title to a row with entries
        let mut goal_rows: IndexMap<String, Vec<String>> = IndexMap::default();
        for ask in &asks_of_this_team {
            let link = format!("{}", ask.link_path.display());

            let goal_title = match &ask.goal_titles[..] {
                [goal_title] => format!("[{goal_title}]({link}#ownership-and-team-asks)"),
                [goal_title, subgoal_title] => {
                    format!("[{subgoal_title}]({link}#ownership-and-team-asks) (part of [{goal_title}]({link}))")
                }
                _ => anyhow::bail!(
                    "expected either 1 or 2 goal titles, not {:?}",
                    ask.goal_titles
                ),
            };

            let row = goal_rows.entry(goal_title).or_insert_with(empty_row);

            let index = ask_headings
                .iter()
                .position(|&h| h == &ask.ask_description)
                .unwrap();

            let text = if !ask.notes.is_empty() {
                &ask.notes
            } else {
                CHECK
            };

            let mut maybe_footnote = |text: &str| -> String {
                if text.len() > FOOTNOTE_LEN {
                    let footnote_index = footnotes.len() + 1;
                    footnotes.push(format!("\\*{footnote_index}: {text} ([from here]({link}))", link = ask.link_path.display()));
                    format!("\\*{footnote_index}")
                } else {
                    text.to_string()
                }
            };

            if !row[index].is_empty() {
                row[index] = format!("{} {}", row[index], maybe_footnote(text));
            } else {
                row[index] = maybe_footnote(text);
            }
        }

        // Sort the goal rows by name (ignoring case).
        goal_rows.sort_by_cached_key(|ask_names, _ask_rows| {  
            ask_names.clone().to_uppercase()
        });

        // Create the table itself.
        let table = {
            let headings = std::iter::once("Goal".to_string())
                .chain(
                    ask_headings
                        .iter()
                        .map(|&ask_kind| format!(
                            "[{team_ask_short}][valid_team_asks]", // HACK: This should not be hardcoded in the code.
                            team_ask_short = config.team_asks[ask_kind].short,
                        ))
                ) // e.g. "discussion and moral support"
                .collect::<Vec<String>>();

            let rows = goal_rows.into_iter().map(|(goal_title, goal_columns)| {
                std::iter::once(goal_title)
                    .chain(goal_columns)
                    .collect::<Vec<String>>()
            });

            std::iter::once(headings).chain(rows).collect::<Vec<_>>()
        };

        write!(output, "{}", util::format_table(&table))?;

        for footnote in footnotes {
            write!(output, "\n\n{}\n", footnote)?;
        }
    }

    Ok(output)
}

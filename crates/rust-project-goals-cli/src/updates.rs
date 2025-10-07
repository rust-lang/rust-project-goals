use chrono::{Datelike, NaiveDate};
use regex::Regex;
use rust_project_goals::re::{HELP_WANTED, TLDR};
use rust_project_goals::spanned::{Result, Span, Spanned};
use rust_project_goals::util::comma;
use rust_project_goals::{goal, markwaydown, spanned, team};
use rust_project_goals_json::GithubIssueState;
use std::path::PathBuf;

mod templates;
use rust_project_goals::gh::issues::ExistingGithubIssue;
use rust_project_goals::gh::{
    issue_id::{IssueId, Repository},
    issues::{checkboxes, ExistingGithubComment},
};
use templates::{HelpWanted, UpdatesGoal};

/// Library function that renders updates as a string without side effects.
/// This is suitable for use from the mdbook preprocessor.
pub fn render_updates(
    cached_issues: &[ExistingGithubIssue],
    repository: &Repository,
    milestone: &str,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
    with_champion_from: Option<&str>,
    use_progress_bar: bool,
) -> Result<String> {
    let milestone_re = Regex::new(r"^\d{4}[hH][12]$").unwrap();
    if !milestone_re.is_match(milestone) {
        spanned::bail_here!(
            "the milestone `{}` does not follow the `$year$semester` format, where $semester is `h1` or `h2`",
            milestone,
        );
    }

    let issues = cached_issues;

    // Load goal documents to extract theme information and optionally filter by team
    let mut milestone_path = PathBuf::from("src");
    milestone_path.push(milestone);
    let goal_documents = goal::goals_in_dir(&milestone_path)?;

    // Create a mapping from issue numbers to themes for flagship goals
    let issue_themes: std::collections::HashMap<u64, String> = goal_documents
        .iter()
        .filter_map(|doc| {
            if let (Some(flagship_theme), Some(tracking_issue)) = (
                doc.metadata.flagship(),
                doc.metadata.tracking_issue.as_ref(),
            ) {
                Some((tracking_issue.number, flagship_theme.trim().to_string()))
            } else {
                None
            }
        })
        .collect();

    // Create mappings for ownership information
    let issue_point_of_contact: std::collections::HashMap<u64, String> = goal_documents
        .iter()
        .filter_map(|doc| {
            doc.metadata
                .tracking_issue
                .as_ref()
                .map(|issue| (issue.number, doc.point_of_contact_for_goal_list()))
        })
        .collect();

    let issue_team_champions: std::collections::HashMap<u64, String> = goal_documents
        .iter()
        .filter_map(|doc| {
            doc.metadata.tracking_issue.as_ref().map(|issue| {
                let teams: std::collections::BTreeSet<&rust_project_goals::team::TeamName> = doc
                    .team_asks
                    .iter()
                    .flat_map(|ask| &ask.teams)
                    .copied()
                    .collect();

                let team_champions: Vec<String> = teams
                    .into_iter()
                    .filter_map(|team| {
                        doc.metadata
                            .champions
                            .get(team)
                            .map(|champion| format!("{} ({})", team, champion.content))
                    })
                    .collect();

                let team_champions_str = if team_champions.is_empty() {
                    "(none)".to_string()
                } else {
                    team_champions.join(", ")
                };

                (issue.number, team_champions_str)
            })
        })
        .collect();

    let issue_task_owners: std::collections::HashMap<u64, String> = goal_documents
        .iter()
        .filter_map(|doc| {
            doc.metadata.tracking_issue.as_ref().map(|issue| {
                let task_owners_str = if doc.task_owners.is_empty() {
                    "(none)".to_string()
                } else {
                    doc.task_owners
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                (issue.number, task_owners_str)
            })
        })
        .collect();

    // Filter issues by champion team if specified
    let filtered_issues: Vec<ExistingGithubIssue> = if let Some(team_name) = with_champion_from {
        let team_name = team::get_team_name(team_name)?
            .ok_or_else(|| spanned::Error::str(format!("unknown team: {}", team_name)))?;

        // Create a set of issue numbers for goals that have a champion from the specified team
        let champion_issue_numbers: std::collections::HashSet<u64> = goal_documents
            .iter()
            .filter(|doc| doc.metadata.champions.contains_key(team_name))
            .filter_map(|doc| {
                doc.metadata
                    .tracking_issue
                    .as_ref()
                    .map(|issue| issue.number)
            })
            .collect();

        issues
            .iter()
            .filter(|issue| champion_issue_numbers.contains(&issue.number))
            .cloned()
            .collect()
    } else {
        issues.to_vec()
    };

    let filter = Filter {
        start_date: match start_date {
            Some(date) => date.clone(),
            None => default_start_date(),
        },
        end_date,
    };

    if use_progress_bar {
        progress_bar::init_progress_bar(filtered_issues.len());
        progress_bar::set_progress_bar_action(
            "Executing",
            progress_bar::Color::Blue,
            progress_bar::Style::Bold,
        );
    }

    let flagship_goals = prepare_goals(
        repository,
        &filtered_issues,
        &filter,
        true,
        use_progress_bar,
        &issue_themes,
        &issue_point_of_contact,
        &issue_team_champions,
        &issue_task_owners,
    )?;
    let other_goals = prepare_goals(
        repository,
        &filtered_issues,
        &filter,
        false,
        use_progress_bar,
        &issue_themes,
        &issue_point_of_contact,
        &issue_team_champions,
        &issue_task_owners,
    )?;
    let updates = templates::Updates::new(milestone.to_string(), flagship_goals, other_goals);

    if use_progress_bar {
        progress_bar::finalize_progress_bar();
    }

    // Render the output using handlebars and return it
    updates.render()
}

fn prepare_goals(
    repository: &Repository,
    issues: &[ExistingGithubIssue],
    filter: &Filter<'_>,
    flagship: bool,
    use_progress_bar: bool,
    issue_themes: &std::collections::HashMap<u64, String>,
    issue_point_of_contact: &std::collections::HashMap<u64, String>,
    issue_team_champions: &std::collections::HashMap<u64, String>,
    issue_task_owners: &std::collections::HashMap<u64, String>,
) -> Result<Vec<UpdatesGoal>> {
    let mut result = vec![];
    // We process flagship and regular goals in two passes, and capture comments differently for flagship goals.
    for issue in issues {
        if flagship != issue.has_flagship_label() {
            continue;
        }

        let issue_id = IssueId {
            repository: repository.clone(),
            number: issue.number,
        };

        let title = &issue.title;

        if use_progress_bar {
            progress_bar::print_progress_bar_info(
                &format!("Issue #{number}", number = issue.number),
                title,
                progress_bar::Color::Green,
                progress_bar::Style::Bold,
            );
        }

        let progress = checkboxes(&issue);

        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| !c.should_hide_from_reports() && filter.matches(c));

        // Prettify the comments' timestamp after using it for sorting.
        for comment in comments.iter_mut() {
            comment.created_at = format!("{}", comment.created_at_date());
        }

        let tldr = tldr(&issue_id, &mut comments)?;

        let (has_help_wanted, help_wanted) = help_wanted(&issue_id, &tldr, &comments)?;

        let why_this_goal = why_this_goal(&issue_id, issue)?;

        let details_summary = match comments.len() {
            0 => String::from("No detailed updates available."),
            1 => String::from("1 detailed update available."),
            len => format!("{len} detailed updates available."),
        };
        result.push(UpdatesGoal {
            title: title.clone(),
            issue_number: issue.number,
            issue_assignees: comma(&issue.assignees),
            issue_url: issue_id.url(),
            issue_link_text: format!("rust-lang/rust-project-goals#{}", issue.number),
            progress,
            has_help_wanted,
            help_wanted,
            is_closed: issue.state == GithubIssueState::Closed,
            details_summary,
            comments,
            tldr,
            why_this_goal,
            needs_separator: true, // updated after sorting
            theme: issue_themes.get(&issue.number).cloned(),
            point_of_contact: issue_point_of_contact
                .get(&issue.number)
                .cloned()
                .unwrap_or_else(|| "(unknown)".to_string()),
            team_champions: issue_team_champions
                .get(&issue.number)
                .cloned()
                .unwrap_or_else(|| "(none)".to_string()),
            task_owners: issue_task_owners
                .get(&issue.number)
                .cloned()
                .unwrap_or_else(|| "(none)".to_string()),
        });

        if use_progress_bar {
            progress_bar::inc_progress_bar();
        }
    }

    // Updates are in a random order, sort them.
    result.sort_by_cached_key(|update| update.title.to_lowercase());

    // Mark the last entry as not needing a separator from its following sibling, it has none.
    if let Some(last) = result.last_mut() {
        last.needs_separator = false;
    }

    Ok(result)
}

/// Search for a TL;DR comment. If one is found, remove it and return the text.
fn tldr(_issue_id: &IssueId, comments: &mut Vec<ExistingGithubComment>) -> Result<Option<String>> {
    // `comments` are sorted by creation date in an ascending order, so we look for the most recent
    // TL;DR comment from the end.
    let Some(index) = comments.iter().rposition(|c| c.body.starts_with(TLDR)) else {
        return Ok(None);
    };

    let comment = comments.remove(index);
    Ok(Some(comment.body[TLDR.len()..].trim().to_string()))
}

/// Search for comments that talk about help being wanted and extract that
fn help_wanted(
    _issue_id: &IssueId,
    tldr: &Option<String>,
    comments: &[ExistingGithubComment],
) -> Result<(bool, Vec<HelpWanted>)> {
    use std::fmt::Write;

    let mut help_wanted = vec![];

    let tldr_has_help_wanted = tldr
        .as_deref()
        .unwrap_or("")
        .lines()
        .any(|line| HELP_WANTED.is_match(line));

    for comment in comments {
        let mut lines = comment.body.split('\n').peekable();

        // Look for a line that says "Help wanted" at the front.
        // Then extract the rest of that line along with subsequent lines until we find a blank line.
        while lines.peek().is_some() {
            while let Some(line) = lines.next() {
                if let Some(c) = HELP_WANTED.captures(line) {
                    let text = c["text"].trim().to_string();
                    if !text.is_empty() {
                        help_wanted.push(HelpWanted { text });
                        break;
                    }
                }
            }

            while let Some(line) = lines.next() {
                if line.trim().is_empty() {
                    break;
                } else {
                    let last = help_wanted.len() - 1;
                    writeln!(&mut help_wanted[last].text, "{line}")?;
                }
            }
        }
    }

    Ok((tldr_has_help_wanted || !help_wanted.is_empty(), help_wanted))
}

fn why_this_goal(issue_id: &IssueId, issue: &ExistingGithubIssue) -> Result<String> {
    let span = Span {
        file: issue_id.url().into(),
        bytes: 0..issue.body.len(),
    };
    let sections = markwaydown::parse_text(Spanned::new(&issue.body, span))?;
    for section in sections {
        if section.title == "Why this goal?" {
            return Ok(section.text.trim().to_string());
        }
    }
    return Ok("".to_string());
}

struct Filter<'f> {
    start_date: NaiveDate,
    end_date: &'f Option<NaiveDate>,
}

impl Filter<'_> {
    fn matches(&self, comment: &ExistingGithubComment) -> bool {
        let date = comment.created_at_date();

        date >= self.start_date
            && match self.end_date {
                Some(end_date) => date <= *end_date,
                None => true,
            }
    }
}

fn default_start_date() -> NaiveDate {
    let date = chrono::Utc::now().date_naive();
    let start_of_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
    start_of_month - chrono::Duration::days(7)
}

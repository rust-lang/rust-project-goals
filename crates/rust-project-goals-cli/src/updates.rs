use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use rust_project_goals::markwaydown;
use rust_project_goals::re::{HELP_WANTED, TLDR};
use rust_project_goals::util::comma;
use rust_project_goals_json::GithubIssueState;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

mod templates;
use rust_project_goals::gh::issues::ExistingGithubIssue;
use rust_project_goals::gh::{
    issue_id::{IssueId, Repository},
    issues::{checkboxes, list_issues_in_milestone, ExistingGithubComment},
};
use templates::{HelpWanted, UpdatesGoal};

pub(crate) fn generate_updates(
    repository: &Repository,
    milestone: &str,
    output_file: Option<&Path>,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
    vscode: bool,
) -> anyhow::Result<()> {
    if output_file.is_none() && !vscode {
        anyhow::bail!("either `--output-file` or `--vscode` must be specified");
    }

    let issues = list_issues_in_milestone(repository, milestone)?;

    let filter = Filter {
        start_date: match start_date {
            Some(date) => date.clone(),
            None => default_start_date(),
        },
        end_date,
    };

    progress_bar::init_progress_bar(issues.len());
    progress_bar::set_progress_bar_action(
        "Executing",
        progress_bar::Color::Blue,
        progress_bar::Style::Bold,
    );

    let flagship_goals = prepare_goals(repository, &issues, &filter, true)?;
    let other_goals = prepare_goals(repository, &issues, &filter, false)?;
    let updates = templates::Updates::new(milestone.to_string(), flagship_goals, other_goals);

    progress_bar::finalize_progress_bar();

    // Render the output using handlebars and write it to the file.
    let output = updates.render()?;

    if let Some(output_file) = output_file {
        std::fs::write(&output_file, output)
            .with_context(|| format!("failed to write to `{}`", output_file.display()))?;
    } else if vscode {
        let mut child = Command::new("code")
            .arg("-")
            .stdin(Stdio::piped())
            .spawn()
            .with_context(|| "failed to spawn `code` process")?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(output.as_bytes())
                .with_context(|| "failed to write to `code` stdin")?;
        }

        child
            .wait()
            .with_context(|| "failed to wait on `code` process")?;
    } else {
        println!("{output}");
    }

    Ok(())
}

fn prepare_goals(
    repository: &Repository,
    issues: &[ExistingGithubIssue],
    filter: &Filter<'_>,
    flagship: bool,
) -> anyhow::Result<Vec<UpdatesGoal>> {
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

        progress_bar::print_progress_bar_info(
            &format!("Issue #{number}", number = issue.number),
            title,
            progress_bar::Color::Green,
            progress_bar::Style::Bold,
        );

        let progress = checkboxes(&issue);

        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| !c.is_automated_comment() && filter.matches(c));
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
            progress,
            has_help_wanted,
            help_wanted,
            is_closed: issue.state == GithubIssueState::Closed,
            details_summary,
            comments,
            tldr,
            why_this_goal,
            needs_separator: true, // updated after sorting
        });

        progress_bar::inc_progress_bar();
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
fn tldr(
    _issue_id: &IssueId,
    comments: &mut Vec<ExistingGithubComment>,
) -> anyhow::Result<Option<String>> {
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
) -> anyhow::Result<(bool, Vec<HelpWanted>)> {
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

fn why_this_goal(issue_id: &IssueId, issue: &ExistingGithubIssue) -> anyhow::Result<String> {
    let sections = markwaydown::parse_text(issue_id.url(), &issue.body)?;
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

use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use rust_project_goals::util::comma;
use rust_project_goals_json::GithubIssueState;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::templates::{self, Updates, UpdatesGoal};
use rust_project_goals::gh::issues::ExistingGithubIssue;
use rust_project_goals::gh::{
    issue_id::{IssueId, Repository},
    issues::{checkboxes, list_issues_in_milestone, ExistingGithubComment},
};

pub async fn updates(
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

    let mut updates = templates::Updates {
        milestone: milestone.to_string(),
        flagship_goals: vec![],
        other_goals: vec![],
    };

    prepare_flagship_goals(repository, &issues, &filter, &mut updates).await?;
    prepare_other_goals(repository, &issues, &filter, &mut updates).await?;

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

async fn prepare_flagship_goals(
    repository: &Repository,
    issues: &[ExistingGithubIssue],
    filter: &Filter<'_>,
    updates: &mut Updates,
) -> anyhow::Result<()> {
    // First process the flagship goals, for which we capture the full text of comments.
    for issue in issues {
        if !issue.has_flagship_label() {
            continue;
        }

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

        let mut summary = String::new();
        for c in comments {
            summary.push_str(&c.body);
            summary.push_str("\n\n");
        }

        updates.flagship_goals.push(UpdatesGoal {
            title: title.clone(),
            issue_number: issue.number,
            issue_assignees: comma(&issue.assignees),
            issue_url: IssueId {
                repository: repository.clone(),
                number: issue.number,
            }
            .url(),
            progress,
            is_closed: issue.state == GithubIssueState::Closed,
            updates_markdown: summary,
        });

        progress_bar::inc_progress_bar();
    }
    Ok(())
}

async fn prepare_other_goals(
    repository: &Repository,
    issues: &[ExistingGithubIssue],
    filter: &Filter<'_>,
    updates: &mut Updates,
) -> anyhow::Result<()> {
    // Next process the remaining goals, for which we generate a summary using an LLVM.
    for issue in issues {
        if issue.has_flagship_label() {
            continue;
        }

        let title = &issue.title;

        progress_bar::print_progress_bar_info(
            &format!("Issue #{number}", number = issue.number),
            title,
            progress_bar::Color::Green,
            progress_bar::Style::Bold,
        );

        // Find the relevant updates that have occurred.
        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| !c.is_automated_comment() && filter.matches(c));

        // Use an LLM to summarize the updates.
        let summary = if comments.len() == 0 {
            format!("No updates in this period.")
        } else {
            format!(
                "[{N} updates available.]({LINK})",
                N = comments.len(),
                LINK = comments.first().unwrap().url,
            )
        };

        let goal = UpdatesGoal {
            title: title.clone(),
            issue_number: issue.number,
            issue_assignees: comma(&issue.assignees),
            issue_url: IssueId {
                repository: repository.clone(),
                number: issue.number,
            }
            .url(),
            is_closed: issue.state == GithubIssueState::Closed,
            updates_markdown: summary,
            progress: checkboxes(&issue),
        };

        updates.other_goals.push(goal);

        progress_bar::inc_progress_bar();
    }
    Ok(())
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

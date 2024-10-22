use std::path::Path;

use anyhow::Context;
use chrono::{Datelike, NaiveDate};

use crate::{
    gh::{
        issue_id::{IssueId, Repository},
        issues::{list_issue_titles_in_milestone, ExistingGithubComment, ExistingIssueState},
    },
    json::checkboxes,
    llm::LargeLanguageModel,
    util::comma,
};

pub async fn updates(
    repository: &Repository,
    milestone: &str,
    output_file: Option<&Path>,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let output_file = match output_file {
        Some(p) => p.to_path_buf(),
        None => Path::new(milestone).with_extension("md"),
    };

    let llm = LargeLanguageModel::new().await;

    let issues = list_issue_titles_in_milestone(repository, milestone)?;

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

    let mut output = String::new();

    for (title, issue) in issues {
        progress_bar::print_progress_bar_info(
            &format!("Issue #{number}", number = issue.number),
            &title,
            progress_bar::Color::Green,
            progress_bar::Style::Bold,
        );

        let prompt = format!(
            "The following comments are updates to a project goal entitled '{title}'. \
            The goal is assigned to {people} ({assignees}). \
            Please create a 1 paragraph summary of these updates. \
            Do not restate the goal title or give a generic introduction. \
            Write the update in the third person. \
            ",
            people = if issue.assignees.len() == 1 {
                "1 person".to_string()
            } else {
                format!("{} people", issue.assignees.len())
            },
            assignees = comma(&issue.assignees),
        );

        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| filter.matches(c));

        let progress = checkboxes(&issue);
        let (completed, total) = progress.completed_total();
        let status_badge = match issue.state {
            ExistingIssueState::Open => {
                format!(
                    "![Status: {percent}%](https://img.shields.io/badge/Status-{percent}%25-green)",
                    percent = completed * 100 / total
                )
            }
            ExistingIssueState::Closed if completed == total => {
                format!("![Status: Complete](https://img.shields.io/badge/Status-Completed-green)")
            }
            ExistingIssueState::Closed => {
                format!("![Status: Incomplete](https://img.shields.io/badge/Status-Incomplete%20-yellow)")
            }
        };

        let issue_id = IssueId {
            repository: repository.clone(),
            number: issue.number,
        };

        writeln!(output)?;
        writeln!(output)?;
        writeln!(output)?;
        writeln!(
            output,
            "# [Issue #{number}]({url}): {title} {status_badge}",
            number = issue.number,
            url = issue_id.url(),
        )?;
        writeln!(output)?;
        writeln!(
            output,
            "* Assigned to: {assignees}",
            assignees = comma(&issue.assignees)
        )?;
        writeln!(output)?;

        if comments.len() > 0 {
            let updates: String = comments.iter().map(|c| format!("\n{}\n", c.body)).collect();
            let summary = llm.query(&prompt, &updates).await?;
            writeln!(output)?;
            writeln!(output, "{}", summary)?;
        } else {
            writeln!(output)?;
            writeln!(output, "No updates in this period.")?;
        }

        progress_bar::inc_progress_bar();
    }

    std::fs::write(&output_file, output)
        .with_context(|| format!("failed to write to `{}`", output_file.display()))?;

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

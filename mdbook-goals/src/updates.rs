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

    writeln!(output, "INTRO")?;

    // First process the flagship goals. These are handled differently.
    writeln!(output, "## Flagship goals")?;
    for (title, issue) in &issues {
        if !issue.has_flagship_label() {
            continue;
        }

        progress_bar::print_progress_bar_info(
            &format!("Issue #{number}", number = issue.number),
            &title,
            progress_bar::Color::Green,
            progress_bar::Style::Bold,
        );

        let issue_id = IssueId {
            repository: repository.clone(),
            number: issue.number,
        };

        writeln!(output, "### {title}")?;
        writeln!(
            output,
            "**Tracked in [#{number}]({url}); assigned to {assignees}",
            number = issue.number,
            url = issue_id.url(),
            assignees = comma(&issue.assignees),
        )?;

        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| filter.matches(c));

        for comment in comments {
            writeln!(
                output,
                "Update by {author} on [{date}]({url}):\n\n{body}",
                author = comment.author,
                date = comment.created_at_date().format("%m %d"),
                body = comment.body,
                url = comment.url,
            )?;
        }
    }

    // Next process the remaining goals, for which we generate a table.
    writeln!(output, "## Other goals")?;
    writeln!(output, "<table>")?;
    for (title, issue) in &issues {
        if issue.has_flagship_label() {
            continue;
        }

        progress_bar::print_progress_bar_info(
            &format!("Issue #{number}", number = issue.number),
            &title,
            progress_bar::Color::Green,
            progress_bar::Style::Bold,
        );

        let prompt = format!(
            "The following comments are updates to a project goal entitled '{title}'. \
            The goal is assigned to {people} ({assignees}). \
            Summarize the updates with a list of one or two bullet points, each one sentence. \
            Write the update in the third person. \
            Format the bullet points as markdown with each bullet point beginning with `* `. \
            Do not respond with anything but the bullet points. \
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
                format!("<progress value="{completed}" max="{total}")
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

        writeln!(output, "<tr>")?;
        writeln!(output, "<th>")?;
        writeln!(output, "[#{number}]({url}", number = issue.number, url = issue_id.url())?;
        writeln!(output, "</th>")?;
        writeln!(output, "<th>")?;
        writeln!(output, "{title}");
        writeln!(output, "</th>")?;
        writeln!(output, "<th>")?;
        writeln!(output, "{status_badge}");
        writeln!(output, "</th>")?;
        writeln!(output, "</tr>")?;
        writeln!(output, "<tr>")?;
        writeln!(output, "<td colspan='3'>")?;
        writeln!(
            output,
            "Assigned to: {assignees}",
            assignees = comma(&issue.assignees)
        )?;
        writeln!(output, "</td>")?;
        writeln!(output, "</tr>")?;
        writeln!(output, "<tr>")?;
        if comments.len() > 0 {
            let updates: String = comments.iter().map(|c| format!("\n{}\n", c.body)).collect();
            let summary = llm.query(&prompt, &updates).await?;
            writeln!(output, "{}", summary)?;
        } else {
            writeln!(output)?;
            writeln!(output, "* No updates in this period.")?;
        }
        writeln!(output, "</tr>")?;

        progress_bar::inc_progress_bar();
    }
    writeln!(output, "</table>")?;

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

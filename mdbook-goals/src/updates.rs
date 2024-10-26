use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use std::path::Path;

use crate::{
    gh::{
        issue_id::{IssueId, Repository},
        issues::{list_issue_titles_in_milestone, ExistingGithubComment},
    },
    json::checkboxes,
    llm::LargeLanguageModel,
    templates::{self, UpdatesFlagshipGoal, UpdatesFlagshipGoalUpdate, UpdatesOtherGoal},
    util::comma,
};

pub async fn updates(
    repository: &Repository,
    milestone: &str,
    output_file: Option<&Path>,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
) -> anyhow::Result<()> {
    let _templates = templates::Templates::new()?;

    let output_file = match output_file {
        Some(p) => p.to_path_buf(),
        None => Path::new(milestone).with_extension("md"),
    };

    let llm = LargeLanguageModel::new().await;

    let mut issues = list_issue_titles_in_milestone(repository, milestone)?;
    issues.clear();

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

    // First process the flagship goals, for which we capture the full text of comments.
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

        let progress = checkboxes(&issue);

        updates.flagship_goals.push(UpdatesFlagshipGoal {
            title: title.clone(),
            issue_number: issue.number,
            issue_assignees: comma(&issue.assignees),
            issue_url: IssueId {
                repository: repository.clone(),
                number: issue.number,
            }
            .url(),
            progress,
            updates: issue
                .comments
                .iter()
                .filter(|c| filter.matches(c))
                .map(|c| UpdatesFlagshipGoalUpdate {
                    author: c.author.clone(),
                    date: c.created_at_date().format("%m %d").to_string(),
                    update: c.body.clone(),
                    url: c.url.clone(),
                })
                .collect(),
        });

        progress_bar::inc_progress_bar();
    }

    // Next process the remaining goals, for which we generate a summary using an LLVM.
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

        // Use an LLM to summarize the updates.
        let summary = {
            let prompt = format!(
                "The following comments are updates to a project goal entitled '{title}'. \
                The goal is assigned to {people} ({assignees}). \
                Summarize the updates with a list of one or two bullet points, each one sentence. \
                Write the update in the third person and do not use pronouns when referring to people. \
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
            if comments.len() > 0 {
                let updates: String = comments.iter().map(|c| format!("\n{}\n", c.body)).collect();
                llm.query(&prompt, &updates).await?
            } else {
                format!("* No updates in this period.")
            }
        };

        updates.other_goals.push(UpdatesOtherGoal {
            title: title.clone(),
            issue_number: issue.number,
            issue_assignees: comma(&issue.assignees),
            issue_url: IssueId {
                repository: repository.clone(),
                number: issue.number,
            }
            .url(),
            updates_markdown: summary,
            progress: checkboxes(&issue),
        });

        progress_bar::inc_progress_bar();
    }

    progress_bar::finalize_progress_bar();

    // Render the output using handlebars and write it to the file.
    let output = updates.render()?;
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

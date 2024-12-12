use anyhow::Context;
use chrono::{Datelike, NaiveDate};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use rust_project_goals::util::comma;

use crate::gh::issues::ExistingGithubIssue;
use crate::templates::{Updates, UpdatesGoal};
use crate::{
    gh::{
        issue_id::{IssueId, Repository},
        issues::{list_issue_titles_in_milestone, ExistingGithubComment, ExistingIssueState},
    },
    json::checkboxes,
    llm::LargeLanguageModel,
    templates,
};

const QUICK_UPDATES: &[&str] = &[
    "Jack and Jill went up the hill",
    "To fetch a pail of water",
    "Jack fell down and broke his crown",
    "And Jill came tumbling after.",
    "Up Jack got and home did trot,",
    "As fast as he could caper;",
    "Went to bed to mend his head",
    "With vinegar and brown paper.",
    "Jill came in and she did grin",
    "To see his paper plaster;",
    "Mother, vex’d, did whip her next",
    "For causing Jack's disaster.",
];

fn comments_forever() -> impl Iterator<Item = &'static str> {
    QUICK_UPDATES.iter().copied().cycle()
}

pub async fn updates(
    repository: &Repository,
    milestone: &str,
    output_file: Option<&Path>,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
    quick: bool,
    vscode: bool,
    model_id: Option<&str>,
    region: Option<&str>,
) -> anyhow::Result<()> {
    if output_file.is_none() && !vscode {
        anyhow::bail!("either `--output-file` or `--vscode` must be specified");
    }

    let llm = LargeLanguageModel::new(model_id, region).await?;

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

    let mut updates = templates::Updates {
        milestone: milestone.to_string(),
        flagship_goals: vec![],
        other_goals_with_updates: vec![],
        other_goals_without_updates: vec![],
    };

    prepare_flagship_goals(repository, &issues, &filter, &llm, quick, &mut updates).await?;
    prepare_other_goals(repository, &issues, &filter, &llm, quick, &mut updates).await?;

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
    issues: &BTreeMap<String, ExistingGithubIssue>,
    filter: &Filter<'_>,
    llm: &LargeLanguageModel,
    quick: bool,
    updates: &mut Updates,
) -> anyhow::Result<()> {
    // First process the flagship goals, for which we capture the full text of comments.
    for (title, issue) in issues {
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

        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| filter.matches(c));

        let summary: String = if comments.len() == 0 {
            format!("No updates in this period.")
        } else if quick {
            QUICK_UPDATES.iter().copied().collect()
        } else {
            let prompt = format!(
                "The following comments are updates to a project goal entitled '{title}'. \
                The goal is assigned to {people} ({assignees}). \
                Summarize the major developments, writing for general Rust users. \
                Write the update in the third person and do not use pronouns when referring to people. \
                Do not respond with anything but the summary paragraphs. \
                ",
                people = if issue.assignees.len() == 1 {
                    "1 person".to_string()
                } else {
                    format!("{} people", issue.assignees.len())
                },
                assignees = comma(&issue.assignees),
            );
            let updates: String = comments.iter().map(|c| format!("\n{}\n", c.body)).collect();
            llm.query(&prompt, &updates)
                .await
                .with_context(|| format!("making request to LLM failed"))?
        };

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
            is_closed: issue.state == ExistingIssueState::Closed,
            updates_markdown: summary,
        });

        progress_bar::inc_progress_bar();
    }
    Ok(())
}

async fn prepare_other_goals(
    repository: &Repository,
    issues: &BTreeMap<String, ExistingGithubIssue>,
    filter: &Filter<'_>,
    llm: &LargeLanguageModel,
    quick: bool,
    updates: &mut Updates,
) -> anyhow::Result<()> {
    // Next process the remaining goals, for which we generate a summary using an LLVM.
    let mut quick_comments = comments_forever();
    for (title, issue) in issues {
        if issue.has_flagship_label() {
            continue;
        }

        progress_bar::print_progress_bar_info(
            &format!("Issue #{number}", number = issue.number),
            &title,
            progress_bar::Color::Green,
            progress_bar::Style::Bold,
        );

        // Find the relevant updates that have occurred.
        let mut comments = issue.comments.clone();
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| filter.matches(c));

        // Use an LLM to summarize the updates.
        let summary = if comments.len() == 0 {
            format!("* No updates in this period.")
        } else if quick {
            let num_comments = std::cmp::min(comments.len(), 3);
            quick_comments
                .by_ref()
                .take(num_comments)
                .map(|c| format!("* {c}\n"))
                .collect()
        } else {
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
            let updates: String = comments.iter().map(|c| format!("\n{}\n", c.body)).collect();
            llm.query(&prompt, &updates).await?
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
            is_closed: issue.state == ExistingIssueState::Closed,
            updates_markdown: summary,
            progress: checkboxes(&issue),
        };

        if comments.len() > 0 {
            updates.other_goals_with_updates.push(goal);
        } else {
            updates.other_goals_without_updates.push(goal);
        }

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

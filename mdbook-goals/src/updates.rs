use std::path::Path;

use anyhow::Context;
use chrono::{Datelike, NaiveDate};

use crate::{
    gh::{
        issue_id::Repository,
        issues::{list_issue_titles_in_milestone, ExistingGithubComment},
    },
    util::comma,
};

pub fn updates(
    repository: &Repository,
    milestone: &str,
    output_directory: &Path,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let issues = list_issue_titles_in_milestone(repository, milestone)?;

    let filter = Filter {
        start_date: match start_date {
            Some(date) => date.clone(),
            None => default_start_date(),
        },
        end_date,
    };

    std::fs::create_dir_all(output_directory)
        .with_context(|| format!("creating directory `{}`", output_directory.display()))?;

    for (title, issue) in issues {
        let mut output_text = String::new();
        writeln!(
            output_text,
            "What follows is a series of updates related to a project goal entitled {title}. \
            The goal is assigned to {people} ({assignees}). \
            Please create a short 1-2 paragraph summary of these updates suitable for inclusion in a blog post.
            Write the update in the third person. \
            UPDATES START HERE:",
            people = if issue.assignees.len() == 1 { "1 person".to_string() } else { format!("{} people", issue.assignees.len()) },
            assignees = comma(&issue.assignees),
        )?;

        let mut comments = issue.comments;
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| filter.matches(c));

        writeln!(output_text)?;
        if comments.len() == 0 {
            writeln!(
                output_text,
                "No updates since {date}.",
                date = filter.start_date
            )?;
        } else {
            for comment in comments {
                writeln!(output_text, "\n{body}\n", body = comment.body)?;
            }
        }
        let output_file = output_directory
            .join(issue.number.to_string())
            .with_extension("md");
        std::fs::write(&output_file, output_text)
            .with_context(|| format!("writing to `{}`", output_file.display()))?;
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

use chrono::{Datelike, NaiveDate};

use crate::{
    gh::issues::{list_issue_titles_in_milestone, ExistingGithubComment},
    util::comma,
};

pub fn updates(
    repo: &str,
    milestone: &str,
    start_date: &Option<NaiveDate>,
    end_date: &Option<NaiveDate>,
) -> anyhow::Result<()> {
    let issues = list_issue_titles_in_milestone(repo, milestone)?;

    let filter = Filter {
        start_date: match start_date {
            Some(date) => date.clone(),
            None => default_start_date(),
        },
        end_date,
    };

    for (title, issue) in issues {
        let total_updates = issue
            .comments
            .iter()
            .filter(|c| !c.is_automated_comment())
            .count();

        println!(
            "# {title} (#{number})",
            title = title,
            number = issue.number
        );
        println!("");
        println!("| Metadata | |");
        println!("| --- | --- |");
        println!(
            "| Assigned to | {assignees} |",
            assignees = comma(&issue.assignees),
        );
        println!("| State | {state} |", state = issue.state);
        println!("| Total updates | {total_updates} |");
        println!(
            "| Date of most recent update | {date} |",
            date = issue
                .comments
                .last()
                .map(|c| c.created_at_date().to_string())
                .unwrap_or("none".to_string())
        );

        let mut comments = issue.comments;
        comments.sort_by_key(|c| c.created_at.clone());
        comments.retain(|c| filter.matches(c));

        println!();
        if comments.len() == 0 {
            println!("No updates since {date}.", date = filter.start_date);
        } else {
            for comment in comments {
                println!(
                    "## Update by {author} from {created} ([link]({url}))",
                    author = comment.author,
                    created = comment.created_at_date(),
                    url = comment.url,
                );
                println!("\n{body}\n", body = comment.body);
            }
        }
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

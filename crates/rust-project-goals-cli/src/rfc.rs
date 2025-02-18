use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use anyhow::Context;
use regex::Regex;

use rust_project_goals::{
    gh::{
        issue_id::{IssueId, Repository},
        issues::{
            change_milestone, create_comment, create_issue, fetch_issue, fetch_issue_by_title,
            lock_issue, sync_assignees, FLAGSHIP_LABEL, LOCK_TEXT,
        },
        labels::GhLabel,
    },
    goal::{self, GoalDocument, GoalPlan, ParsedOwners},
    team::{get_person_data, TeamName},
};

fn validate_path(path: &Path) -> anyhow::Result<String> {
    if !path.is_dir() {
        return Err(anyhow::anyhow!(
            "RFC path should be a directory like src/2024h2"
        ));
    };

    if path.is_absolute() {
        return Err(anyhow::anyhow!("RFC path should be relative"));
    }

    let timeframe = path
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid path `{}`", path.display()))?;

    Ok(timeframe.to_string())
}

pub fn generate_comment(path: &Path) -> anyhow::Result<()> {
    let _ = validate_path(path)?;
    let goal_documents = goal::goals_in_dir(path)?;
    let teams_with_asks = teams_with_asks(&goal_documents);

    for team_name in teams_with_asks {
        let team_data = team_name.data();

        println!("\n## {}\n", team_data.name);

        let (leads, members): (Vec<_>, Vec<_>) = team_data.members.iter().partition(|m| m.is_lead);

        for lead in leads {
            println!("* [ ] @{} (required, lead)", lead.github);
        }

        for member in members {
            println!("* [ ] {} (optional)", member.github);
        }
    }

    Ok(())
}

pub fn generate_rfc(path: &Path) -> anyhow::Result<()> {
    let timeframe = &validate_path(path)?;

    // run mdbook build
    Command::new("mdbook").arg("build").status()?;

    // find the markdown output
    let generated_path = PathBuf::from("book/markdown")
        .join(timeframe)
        .join("index.md");
    if !generated_path.exists() {
        return Err(anyhow::anyhow!(
            "no markdown generated at {}",
            generated_path.display()
        ));
    }

    let generated_text = std::fs::read_to_string(&generated_path).with_context(|| {
        format!(
            "reading generated markdown from `{}`",
            generated_path.display()
        )
    })?;

    let regex = Regex::new(r"\]\(([^(]*)\.md(#[^)]*)?\)").unwrap();

    let result = regex.replace_all(
        &generated_text,
        format!("](https://rust-lang.github.io/rust-project-goals/{timeframe}/$1.html$2)"),
    );

    println!("{result}");

    Ok(())
}

pub fn generate_issues(
    repository: &Repository,
    path: &Path,
    commit: bool,
    sleep: u64,
) -> anyhow::Result<()> {
    // Verify the `gh` client is installed to compute which actions need to be taken in the repo.
    let sanity_check = Command::new("gh").arg("--version").output();
    if sanity_check.is_err() {
        return Err(anyhow::anyhow!(
            "The github `gh` client is missing and needs to be installed and configured with a token."
        ));
    }

    // Hacky but works: we loop because after creating the issue, we sometimes have additional sync to do,
    // and it's easier this way.
    loop {
        let timeframe = validate_path(path)?;

        let mut goal_documents = goal::goals_in_dir(path)?;
        goal_documents.retain(|gd| gd.is_not_not_accepted());

        let teams_with_asks = teams_with_asks(&goal_documents);
        let mut actions = initialize_labels(repository, &teams_with_asks)?;
        actions.extend(initialize_issues(repository, &timeframe, &goal_documents)?);

        if actions.is_empty() {
            return Ok(());
        }

        if commit {
            progress_bar::init_progress_bar(actions.len());
            progress_bar::set_progress_bar_action(
                "Executing",
                progress_bar::Color::Blue,
                progress_bar::Style::Bold,
            );
            for action in actions.into_iter() {
                progress_bar::print_progress_bar_info(
                    "Action",
                    &format!("{}", action),
                    progress_bar::Color::Green,
                    progress_bar::Style::Bold,
                );
                action.execute(repository, &timeframe)?;
                progress_bar::inc_progress_bar();

                std::thread::sleep(Duration::from_millis(sleep));
            }
            progress_bar::finalize_progress_bar();
        } else {
            eprintln!("Actions to be executed:");
            for action in &actions {
                eprintln!("* {action}");
            }
            eprintln!("");
            eprintln!("Use `--commit` to execute the actions.");
            return Ok(());
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GithubIssue<'doc> {
    pub title: String,
    pub assignees: BTreeSet<String>,
    pub body: String,
    pub labels: Vec<String>,
    pub tracking_issue: Option<&'doc IssueId>,
    pub goal_document: &'doc GoalDocument,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GithubAction<'doc> {
    CreateLabel {
        label: GhLabel,
    },

    CreateIssue {
        issue: GithubIssue<'doc>,
    },

    ChangeMilestone {
        number: u64,
        milestone: String,
    },

    Comment {
        number: u64,
        body: String,
    },

    // We intentionally do not sync the issue *text*, because it may have been edited.
    SyncAssignees {
        number: u64,
        remove_owners: BTreeSet<String>,
        add_owners: BTreeSet<String>,
    },

    LockIssue {
        number: u64,
    },

    LinkToTrackingIssue {
        goal_document: &'doc GoalDocument,
        issue_id: IssueId,
    },
}

/// Initializes the required `T-<team>` labels on the repository.
/// Warns if the labels are found with wrong color.
fn initialize_labels(
    repository: &Repository,
    teams_with_asks: &BTreeSet<&TeamName>,
) -> anyhow::Result<BTreeSet<GithubAction<'static>>> {
    const TEAM_LABEL_COLOR: &str = "bfd4f2";

    let mut desired_labels: BTreeSet<_> = teams_with_asks
        .iter()
        .map(|team| {
            let label_name = team.gh_label();

            GhLabel {
                name: label_name,
                color: TEAM_LABEL_COLOR.to_string(),
            }
        })
        .collect();

    desired_labels.insert(GhLabel {
        name: "C-tracking-issue".to_string(),
        color: "f5f1fd".to_string(),
    });

    desired_labels.insert(GhLabel {
        name: FLAGSHIP_LABEL.to_string(),
        color: "5319E7".to_string(),
    });

    for existing_label in GhLabel::list(repository)? {
        desired_labels.remove(&existing_label);
    }

    Ok(desired_labels
        .into_iter()
        .map(|label| GithubAction::CreateLabel { label })
        .collect())
}

/// Initializes the required `T-<team>` labels on the repository.
/// Warns if the labels are found with wrong color.
fn initialize_issues<'doc>(
    repository: &Repository,
    timeframe: &str,
    goal_documents: &'doc [GoalDocument],
) -> anyhow::Result<BTreeSet<GithubAction<'doc>>> {
    // the set of issues we want to exist
    let desired_issues: BTreeSet<GithubIssue> = goal_documents
        .iter()
        .map(|goal_document| issue(timeframe, goal_document))
        .collect::<anyhow::Result<_>>()?;

    let mut actions = BTreeSet::new();
    for desired_issue in desired_issues {
        let existing_issue = if let Some(tracking_issue) = desired_issue.tracking_issue {
            Some(fetch_issue(repository, tracking_issue.number)?)
        } else {
            fetch_issue_by_title(repository, &desired_issue.title)?
        };

        match existing_issue {
            Some(existing_issue) => {
                if existing_issue.assignees != desired_issue.assignees {
                    actions.insert(GithubAction::SyncAssignees {
                        number: existing_issue.number,
                        remove_owners: existing_issue
                            .assignees
                            .difference(&desired_issue.assignees)
                            .cloned()
                            .collect(),
                        add_owners: desired_issue
                            .assignees
                            .difference(&existing_issue.assignees)
                            .cloned()
                            .collect(),
                    });
                }

                if existing_issue.milestone.as_ref().map(|m| m.title.as_str()) != Some(timeframe) {
                    actions.insert(GithubAction::ChangeMilestone {
                        number: existing_issue.number,
                        milestone: timeframe.to_string(),
                    });
                    actions.insert(GithubAction::Comment {
                        number: existing_issue.number,
                        body: format!(
                            "This is a continuing project goal, and the updates below \
                            this comment will be for the new period {}",
                            timeframe
                        ),
                    });
                }

                if !existing_issue.was_locked() {
                    actions.insert(GithubAction::LockIssue {
                        number: existing_issue.number,
                    });
                    actions.insert(GithubAction::Comment {
                        number: existing_issue.number,
                        body: LOCK_TEXT.to_string(),
                    });
                }

                let issue_id = IssueId::new(repository.clone(), existing_issue.number);
                if desired_issue.tracking_issue != Some(&issue_id) {
                    actions.insert(GithubAction::LinkToTrackingIssue {
                        goal_document: desired_issue.goal_document,
                        issue_id,
                    });
                }
            }

            None => {
                actions.insert(GithubAction::CreateIssue {
                    issue: desired_issue,
                });
            }
        }
    }

    Ok(actions)
}

fn issue<'doc>(timeframe: &str, document: &'doc GoalDocument) -> anyhow::Result<GithubIssue<'doc>> {
    let mut assignees = BTreeSet::default();
    for username in document.metadata.owner_usernames() {
        if let Some(data) = get_person_data(username)? {
            assignees.insert(data.github_username.clone());
        }
    }

    let mut labels = vec!["C-tracking-issue".to_string()];
    if document.metadata.status.is_flagship {
        labels.push("Flagship Goal".to_string());
    }
    for team in document.teams_with_asks() {
        labels.push(team.gh_label());
    }

    Ok(GithubIssue {
        title: document.metadata.title.clone(),
        assignees,
        body: issue_text(timeframe, document)?,
        labels,
        tracking_issue: document.metadata.tracking_issue.as_ref(),
        goal_document: document,
    })
}

fn issue_text(timeframe: &str, document: &GoalDocument) -> anyhow::Result<String> {
    let mut tasks = vec![];
    for goal_plan in &document.goal_plans {
        tasks.extend(task_items(goal_plan)?);
    }

    let teams = document
        .teams_with_asks()
        .iter()
        .map(|team| team.name_and_link())
        .collect::<Vec<_>>();

    let goal_file = document.link_path.file_stem().unwrap().to_str().unwrap();

    Ok(format!(
        r##"
| Metadata         | |
| --------         | --- |
| Point of contact | {poc} |
| Team(s)          | {teams} |
| Goal document    | [{timeframe}/{goal_file}](https://rust-lang.github.io/rust-project-goals/{timeframe}/{goal_file}.html) |

## Summary

{summary}

## Tasks and status

{tasks}

[Team]: https://img.shields.io/badge/Team%20ask-red
"##,
        poc = &document.metadata.owner_usernames().join(", "),
        teams = teams.join(", "),
        summary = document.summary,
        tasks = tasks.join("\n"),
    ))
}

fn task_items(goal_plan: &GoalPlan) -> anyhow::Result<Vec<String>> {
    use std::fmt::Write;

    let mut tasks = vec![];

    if let Some(title) = &goal_plan.subgoal {
        tasks.push(format!("### {title}"));
    }

    for plan_item in &goal_plan.plan_items {
        let mut description = format!(
            "* {box} {text}",
            box = if plan_item.is_complete() { "[x]" } else { "[ ]" },
            text = plan_item.text
        );

        if let Some(parsed_owners) = plan_item.parse_owners()? {
            match parsed_owners {
                ParsedOwners::TeamAsks(asks) => {
                    let teams: Vec<String> = asks.iter().map(|ask| ask.name_and_link()).collect();

                    write!(description, " ({} ![Team][])", teams.join(", "))?;
                }

                ParsedOwners::Usernames(usernames) => {
                    write!(description, " ({})", usernames.join(", "))?;
                }
            }
        }

        tasks.push(description);
    }

    Ok(tasks)
}

fn teams_with_asks(goal_documents: &[GoalDocument]) -> BTreeSet<&'static TeamName> {
    goal_documents
        .iter()
        .flat_map(|g| &g.team_asks)
        .flat_map(|ask| &ask.teams)
        .copied()
        .collect()
}

impl Display for GithubAction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubAction::CreateLabel {
                label: GhLabel { name, color },
            } => {
                write!(f, "create label `{}` with color `{}`", name, color)
            }
            GithubAction::CreateIssue { issue } => {
                write!(f, "create issue \"{}\"", issue.title)
            }
            GithubAction::ChangeMilestone { number, milestone } => {
                write!(f, "update issue #{} milestone to \"{}\"", number, milestone)
            }
            GithubAction::Comment { number, body } => {
                write!(f, "post comment on issue #{}: \"{}\"", number, body)
            }
            GithubAction::SyncAssignees {
                number,
                remove_owners,
                add_owners,
            } => {
                write!(
                    f,
                    "sync issue #{} ({})",
                    number,
                    remove_owners
                        .iter()
                        .map(|s| format!("-{}", s))
                        .chain(add_owners.iter().map(|s| format!("+{}", s)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            GithubAction::LockIssue { number } => {
                write!(f, "lock issue #{}", number)
            }
            GithubAction::LinkToTrackingIssue {
                goal_document,
                issue_id,
            } => {
                write!(
                    f,
                    "link issue {issue_id:?} to the markdown document at {}",
                    goal_document.path.display()
                )
            }
        }
    }
}

impl GithubAction<'_> {
    pub fn execute(self, repository: &Repository, timeframe: &str) -> anyhow::Result<()> {
        match self {
            GithubAction::CreateLabel { label } => {
                label.create(repository)?;
                Ok(())
            }

            GithubAction::CreateIssue {
                issue:
                    GithubIssue {
                        title,
                        assignees,
                        body,
                        labels,
                        tracking_issue: _,
                        goal_document: _,
                    },
            } => {
                create_issue(repository, &body, &title, &labels, &assignees, timeframe)?;

                // Note: the issue is not locked, but we will reloop around later.

                Ok(())
            }

            GithubAction::ChangeMilestone { number, milestone } => {
                change_milestone(repository, number, &milestone)?;
                Ok(())
            }

            GithubAction::Comment { number, body } => {
                create_comment(repository, number, &body)?;
                Ok(())
            }

            GithubAction::SyncAssignees {
                number,
                remove_owners,
                add_owners,
            } => {
                sync_assignees(repository, number, &remove_owners, &add_owners)?;
                Ok(())
            }

            GithubAction::LockIssue { number } => lock_issue(repository, number),

            GithubAction::LinkToTrackingIssue {
                goal_document,
                issue_id: number,
            } => goal_document.link_issue(number),
        }
    }
}

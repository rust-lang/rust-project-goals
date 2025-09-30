use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use regex::Regex;

use rust_project_goals::{
    gh::{
        issue_id::{IssueId, Repository},
        issues::{
            change_milestone, change_title, create_comment, create_issue, fetch_issue,
            list_issues_in_milestone, lock_issue, sync_assignees, sync_labels, update_issue_body,
            CONTINUING_GOAL_PREFIX, FLAGSHIP_LABEL, LOCK_TEXT,
        },
        labels::GhLabel,
    },
    goal::{self, GoalDocument, GoalPlan, ParsedOwners},
    spanned::{self, Context, Error, Result, Spanned},
    team::{get_person_data, TeamName},
};

fn validate_path(path: &Path) -> Result<String> {
    if !path.is_dir() {
        spanned::bail_here!("RFC path should be a directory like src/2024h2");
    };

    if path.is_absolute() {
        spanned::bail_here!("RFC path should be relative");
    }

    let timeframe = path
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .ok_or_else(|| Error::str(format!("invalid path `{}`", path.display())))?;

    Ok(timeframe.to_string())
}

pub fn generate_comment(path: &Path) -> Result<()> {
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

pub fn generate_rfc(path: &Path) -> Result<()> {
    let timeframe = &validate_path(path)?;

    // run mdbook build
    Command::new("mdbook").arg("build").status()?;

    // find the markdown output
    let generated_path = PathBuf::from("book/markdown")
        .join(timeframe)
        .join("index.md");
    if !generated_path.exists() {
        spanned::bail_here!("no markdown generated at {}", generated_path.display());
    }

    let generated_text = std::fs::read_to_string(&generated_path)
        .with_path_context(&generated_path, "reading generated markdown")?;

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
) -> Result<()> {
    // Verify the `gh` client is installed to compute which actions need to be taken in the repo.
    let sanity_check = Command::new("gh").arg("--version").output();
    if sanity_check.is_err() {
        spanned::bail_here!(
            "The github `gh` client is missing and needs to be installed and configured with a token."
        );
    }

    // Hacky but works: we loop because after creating the issue, we sometimes have additional sync to do,
    // and it's easier this way.
    let mut iteration_count = 0;
    const MAX_ITERATIONS: usize = 10;

    loop {
        iteration_count += 1;
        if iteration_count > MAX_ITERATIONS {
            spanned::bail_here!(
                "Fixed point iteration failed to converge after {} iterations. \
                 This may indicate duplicate tracking issues assigned to multiple goals.",
                MAX_ITERATIONS
            );
        }
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
            let mut success = 0;
            for action in actions.into_iter() {
                progress_bar::print_progress_bar_info(
                    "Action",
                    &format!("{}", action),
                    progress_bar::Color::Green,
                    progress_bar::Style::Bold,
                );
                if let Err(e) = action.execute(repository, &timeframe) {
                    progress_bar::print_progress_bar_info(
                        "Error",
                        &format!("{}", e),
                        progress_bar::Color::Red,
                        progress_bar::Style::Bold,
                    );
                } else {
                    success += 1;
                }
                progress_bar::inc_progress_bar();

                std::thread::sleep(Duration::from_millis(sleep));
            }
            progress_bar::finalize_progress_bar();
            if success == 0 {
                spanned::bail_here!("all actions failed, aborting")
            }
            
            // Clear the cached issues since we just modified them
            if let Err(e) = rust_project_goals::gh::issues::clear_milestone_issues_cache(&timeframe) {
                eprintln!("Warning: Failed to clear issues cache: {}", e);
            }
        } else {
            eprintln!("Actions to be executed:");
            for action in &actions {
                eprintln!("* {action}");
            }
            eprintln!("");
            eprintln!("Use `--commit` to execute the actions.");
            return Ok(());
        }

        eprintln!("Waiting for github commands to propagate.");
        std::thread::sleep(Duration::from_millis(1000));
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

    ChangeTitle {
        number: u64,
        title: String,
    },

    ChangeMilestone {
        number: u64,
        milestone: String,
    },

    Comment {
        number: u64,
        body: String,
    },

    UpdateIssueBody {
        number: u64,
        body: String,
    },

    // We intentionally do not sync the issue *text*, because it may have been edited.
    SyncAssignees {
        number: u64,
        remove_owners: BTreeSet<String>,
        add_owners: BTreeSet<String>,
    },

    SyncLabels {
        number: u64,
        remove_labels: BTreeSet<String>,
        add_labels: BTreeSet<String>,
    },

    LockIssue {
        number: u64,
    },
}

/// Initializes the required `T-<team>` labels on the repository.
/// Warns if the labels are found with wrong color.
fn initialize_labels(
    repository: &Repository,
    teams_with_asks: &BTreeSet<&TeamName>,
) -> Result<BTreeSet<GithubAction<'static>>> {
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
) -> Result<BTreeSet<GithubAction<'doc>>> {
    // the set of issues we want to exist
    let desired_issues: BTreeSet<GithubIssue> = goal_documents
        .iter()
        .map(|goal_document| issue(timeframe, goal_document))
        .collect::<Result<_>>()?;

    // Check for duplicate tracking issues
    let mut tracking_issue_counts = std::collections::HashMap::new();
    for issue in &desired_issues {
        if let Some(tracking_issue) = issue.tracking_issue {
            let count = tracking_issue_counts
                .entry(tracking_issue.number)
                .or_insert(0);
            *count += 1;
        }
    }

    for (issue_number, count) in tracking_issue_counts {
        if count > 1 {
            let goals_with_issue: Vec<_> = desired_issues
                .iter()
                .filter(|issue| issue.tracking_issue.map(|ti| ti.number) == Some(issue_number))
                .map(|issue| issue.goal_document.path.display().to_string())
                .collect();

            spanned::bail_here!(
                "Tracking issue #{} is assigned to {} goals: {}. \
                 Each tracking issue can only be assigned to one goal.",
                issue_number,
                count,
                goals_with_issue.join(", ")
            );
        }
    }

    // the list of existing issues in the target milestone
    let milestone_issues = list_issues_in_milestone(repository, timeframe)?;

    let mut actions = BTreeSet::new();

    // Go through each of the issues we want to exist (derived from the goals defined in the target folder)
    for desired_issue in desired_issues {
        // Check if we already created a tracking issue...
        //
        let existing_issue = if let Some(tracking_issue) = desired_issue.tracking_issue {
            // a. We first check if there is a declared tracking issue in the markdown file.
            // If so, check if we've already loaded its data.
            if let Some(issue) = milestone_issues
                .iter()
                .find(|issue| issue.number == tracking_issue.number)
            {
                // If so, reuse it to avoid latency.
                Some(issue.clone())
            } else {
                // If not, load its information from the repository by number.
                let existing_issue =
                    fetch_issue(repository, tracking_issue.number).map_err(|e| {
                        e.wrap_str(Spanned::here(format!(
                            "error while fetching declared tracking issue {} for goal {}",
                            tracking_issue.number,
                            desired_issue.goal_document.path.display(),
                        )))
                    })?;
                Some(existing_issue)
            }
        } else {
            // b. If the markdown does not have a declared tracking issue, then we can search through
            // the issues in the milestone for one with the correct title.
            // We could also do a fresh GH query for an issue with the desired title
            // but that is slower.
            //
            // This addresses a kind of awkward gap in our handling-- when a new project goal
            // is created, we first create an issue for it, then do a loop and execute again.
            // This second time, we will find the issue with the known title, get its
            // number, and put that number into the markdown.
            milestone_issues
                .iter()
                .find(|issue| issue.title == desired_issue.title)
                .cloned()
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

                // Compare labels - convert existing issue labels to strings for comparison
                let existing_label_names: BTreeSet<String> = existing_issue
                    .labels
                    .iter()
                    .map(|label| label.name.clone())
                    .collect();
                let desired_label_names: BTreeSet<String> =
                    desired_issue.labels.iter().cloned().collect();

                if existing_label_names != desired_label_names {
                    actions.insert(GithubAction::SyncLabels {
                        number: existing_issue.number,
                        remove_labels: existing_label_names
                            .difference(&desired_label_names)
                            .cloned()
                            .collect(),
                        add_labels: desired_label_names
                            .difference(&existing_label_names)
                            .cloned()
                            .collect(),
                    });
                }

                if existing_issue.title != desired_issue.title {
                    actions.insert(GithubAction::ChangeTitle {
                        number: existing_issue.number,
                        title: desired_issue.title,
                    });
                }

                if existing_issue.milestone.as_ref().map(|m| m.title.as_str()) != Some(timeframe) {
                    actions.insert(GithubAction::ChangeMilestone {
                        number: existing_issue.number,
                        milestone: timeframe.to_string(),
                    });
                    actions.insert(GithubAction::Comment {
                        number: existing_issue.number,
                        body: format!("{CONTINUING_GOAL_PREFIX} {timeframe}",),
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

                let link_text = goal_document_link(timeframe, &desired_issue.goal_document);
                if !existing_issue.body.contains(&link_text) {
                    // Let's update the tracking issue to the new goal description, while keeping
                    // the old text in case we need it. It's surprisingly hard to get out of GH
                    // otherwise.
                    let body = format!(
                        "{desired_body}\n---\nNote: we have updated the body to match the \
                         {timeframe} goal. Your original text is preserved below. \
                         <details>\n{existing_body}\n</details>",
                        desired_body = desired_issue.body,
                        existing_body = existing_issue.body,
                    );
                    actions.insert(GithubAction::UpdateIssueBody {
                        number: existing_issue.number,
                        body,
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

fn issue<'doc>(timeframe: &str, document: &'doc GoalDocument) -> Result<GithubIssue<'doc>> {
    let mut assignees = BTreeSet::default();
    for username in document.metadata.owner_usernames() {
        if let Some(data) = get_person_data(username)? {
            assignees.insert(data.github_username.clone());
        }
    }

    let mut labels = vec!["C-tracking-issue".to_string()];
    if document.metadata.flagship().is_some() {
        labels.push("Flagship Goal".to_string());
    }
    for team in document.teams_with_asks() {
        labels.push(team.gh_label());
    }

    Ok(GithubIssue {
        title: document.metadata.title.to_string(),
        assignees,
        body: issue_text(timeframe, document)?,
        labels,
        tracking_issue: document.metadata.tracking_issue.as_ref(),
        goal_document: document,
    })
}

fn goal_document_link(timeframe: &str, document: &GoalDocument) -> String {
    let goal_file = document.link_path.file_stem().unwrap().to_str().unwrap();
    format!("[{timeframe}/{goal_file}](https://rust-lang.github.io/rust-project-goals/{timeframe}/{goal_file}.html)")
}

fn issue_text(timeframe: &str, document: &GoalDocument) -> Result<String> {
    let mut tasks = vec![];
    for goal_plan in &document.goal_plans {
        tasks.extend(task_items(goal_plan)?);
    }

    let teams = document
        .teams_with_asks()
        .iter()
        .map(|team| team.name_and_link())
        .collect::<Vec<_>>();

    Ok(format!(
        r##"
| Metadata         | |
| --------         | --- |
| Point of contact | {poc} |
| Team(s)          | {teams} |
| Goal document    | {goaldocument} |

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
        goaldocument = goal_document_link(timeframe, document),
    ))
}

fn task_items(goal_plan: &GoalPlan) -> Result<Vec<String>> {
    use std::fmt::Write;

    let mut tasks = vec![];

    if let Some(title) = &goal_plan.subgoal {
        tasks.push(format!("### {}", **title));
    }

    for plan_item in &goal_plan.plan_items {
        let mut description = format!(
            "* {box} {text}",
            box = if plan_item.is_complete() { "[x]" } else { "[ ]" },
            text = plan_item.text.content
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
                write!(f, "create issue \"{}\"", issue.title,)
            }
            GithubAction::ChangeMilestone { number, milestone } => {
                write!(f, "update issue #{} milestone to \"{}\"", number, milestone)
            }
            GithubAction::ChangeTitle { number, title } => {
                write!(f, "update issue #{} title to \"{}\"", number, title)
            }
            GithubAction::Comment { number, body } => {
                write!(f, "post comment on issue #{}: \"{}\"", number, body)
            }
            GithubAction::UpdateIssueBody { number, body: _ } => {
                write!(f, "update the body on issue #{} for new milestone", number)
            }
            GithubAction::SyncAssignees {
                number,
                remove_owners,
                add_owners,
            } => {
                write!(
                    f,
                    "sync issue #{} assignees ({})",
                    number,
                    remove_owners
                        .iter()
                        .map(|s| format!("-{}", s))
                        .chain(add_owners.iter().map(|s| format!("+{}", s)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            GithubAction::SyncLabels {
                number,
                remove_labels,
                add_labels,
            } => {
                write!(
                    f,
                    "sync issue #{} labels ({})",
                    number,
                    remove_labels
                        .iter()
                        .map(|s| format!("-{}", s))
                        .chain(add_labels.iter().map(|s| format!("+{}", s)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            GithubAction::LockIssue { number } => {
                write!(f, "lock issue #{}", number)
            }
        }
    }
}

impl GithubAction<'_> {
    pub fn execute(self, repository: &Repository, timeframe: &str) -> Result<()> {
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
                        goal_document,
                    },
            } => {
                let issue_id =
                    create_issue(repository, &body, &title, &labels, &assignees, timeframe)?;

                goal_document.link_issue(issue_id)?;

                Ok(())
            }

            GithubAction::ChangeMilestone { number, milestone } => {
                change_milestone(repository, number, &milestone)?;
                Ok(())
            }

            GithubAction::ChangeTitle { number, title } => {
                change_title(repository, number, &title)?;
                Ok(())
            }

            GithubAction::Comment { number, body } => {
                create_comment(repository, number, &body)?;
                Ok(())
            }

            GithubAction::UpdateIssueBody { number, body } => {
                update_issue_body(repository, number, &body)?;
                Ok(())
            }

            GithubAction::SyncAssignees {
                number,
                remove_owners,
                add_owners,
            } => {
                // NOTE: Swallow errors here because sometimes people are not present in the org.
                // We don't want to stop everything for that.
                sync_assignees(repository, number, &remove_owners, &add_owners)?;
                Ok(())
            }

            GithubAction::SyncLabels {
                number,
                remove_labels,
                add_labels,
            } => {
                sync_labels(repository, number, &remove_labels, &add_labels)?;
                Ok(())
            }

            GithubAction::LockIssue { number } => lock_issue(repository, number),
        }
    }
}

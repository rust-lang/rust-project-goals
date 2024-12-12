use anyhow::Context;
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;
use rust_project_goals::gh::issue_id::Repository;

mod rfc;
mod generate_json;
mod team_repo;
mod templates;
mod updates;

#[derive(StructOpt, Debug)]
#[structopt(about = "Project goal preprocessor")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,

    /// Repository to use if applicable
    #[structopt(long, default_value = "rust-lang/rust-project-goals")]
    repository: Repository,
}

#[derive(StructOpt, Debug)]
#[allow(dead_code)]
enum Command {
    /// Print the comment required to initiate FCP
    FCP { path: PathBuf },

    /// Print the RFC text to stdout
    RFC { path: PathBuf },

    /// Use `gh` CLI tool to create issues on the rust-lang/rust-project-goals repository
    Issues {
        path: PathBuf,

        /// Number of milliseconds to pause between github commands
        /// to avoid rate limiting
        #[structopt(long, default_value = "500")]
        sleep: u64,

        /// Without this option, no action is taken.
        #[structopt(long)]
        commit: bool,
    },

    /// Generate the project-goal-owners team based on the owners found in `paths`.
    TeamRepo {
        /// Paths to the directories containing the goals (e.g., `src/2024h2`)
        #[structopt(required = true, min_values = 1)]
        path: Vec<PathBuf>,

        /// Paths to the teams repository checkout
        #[structopt(required = true, long = "team-repo")]
        team_repo_path: PathBuf,
    },

    /// Checks that the goal documents are well-formed, intended for use within CI
    Check {},

    /// Generate json file with status from tracking issues.
    /// This is intended for storing alongside the book for consumption by external tools.
    Json {
        /// Milestone for which we generate tracking issue data (e.g., `2024h2`).
        milestone: String,

        /// Path to write the json (e.g., `book/html/api/milestone.json`).
        /// If not provided, writes to stdout.
        #[structopt(long)]
        json_path: Option<PathBuf>,
    },

    /// Generate markdown with the list of updates for each tracking issue.
    /// Collects updates
    Updates {
        /// Milestone for which we generate tracking issue data (e.g., `2024h2`).
        milestone: String,

        /// Quick mode does not use an LLM to generate a summary.
        #[structopt(long)]
        quick: bool,

        /// Quick mode does not use an LLM to generate a summary.
        #[structopt(long)]
        vscode: bool,

        /// If specified, write the output into the given file.
        #[structopt(long)]
        output_file: Option<PathBuf>,

        /// Start date for comments.
        /// If not given, defaults to 1 week before the start of this month.
        start_date: Option<chrono::NaiveDate>,

        /// End date for comments.
        /// If not given, no end date.
        end_date: Option<chrono::NaiveDate>,

        /// Set a custom model id for the LLM.
        #[structopt(long)]
        model_id: Option<String>,

        /// Set a custom region.
        #[structopt(long)]
        region: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match &opt.cmd {
        Command::FCP { path } => {
            rfc::generate_comment(&path)?;
        }

        Command::Check {} => {
            check()?;
        }

        Command::RFC { path } => {
            rfc::generate_rfc(&path)?;
        }

        Command::Issues {
            path,
            commit,
            sleep,
        } => {
            rfc::generate_issues(&opt.repository, path, *commit, *sleep)
                .with_context(|| format!("failed to adjust issues; rerun command to resume"))?;
        }

        Command::TeamRepo {
            path,
            team_repo_path,
        } => {
            team_repo::generate_team_repo(&path, team_repo_path)?;
        }

        Command::Json {
            milestone,
            json_path,
        } => {
            generate_json::generate_json(&opt.repository, &milestone, json_path)?;
        }
        Command::Updates {
            milestone,
            output_file,
            start_date,
            end_date,
            quick,
            vscode,
            model_id,
            region,
        } => {
            updates::updates(
                &opt.repository,
                milestone,
                output_file.as_deref(),
                start_date,
                end_date,
                *quick,
                *vscode,
                model_id.as_deref(),
                region.as_deref(),
            )
            .await?;
        }
    }

    Ok(())
}

fn check() -> anyhow::Result<()> {
    // Look for all directories like `2024h2` or `2025h1` and load goals from those directories.
    let regex = Regex::new(r"\d\d\d\dh[12]")?;

    for entry in WalkDir::new("src") {
        let entry = entry?;

        if !entry.file_type().is_dir() {
            continue;
        }

        let Some(name) = entry.file_name().to_str() else {
            continue;
        };

        if !regex.is_match(name) {
            continue;
        }

        let _goals = rust_project_goals::goal::goals_in_dir(entry.path())?;
    }

    Ok(())
}

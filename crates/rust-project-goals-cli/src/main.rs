use anyhow::{bail, Context};
use clap::Parser;
use regex::Regex;
use rust_project_goals::gh::issue_id::Repository;
use rust_project_goals_llm::UpdateArgs;
use std::path::PathBuf;
use walkdir::WalkDir;

mod cfp;
mod generate_json;
mod rfc;
mod team_repo;

#[derive(clap::Parser, Debug)]
#[structopt(about = "Project goal preprocessor")]
struct Opt {
    #[command(subcommand)]
    cmd: Command,

    /// Repository to use if applicable
    #[arg(long, default_value = "rust-lang/rust-project-goals")]
    repository: Repository,
}

#[derive(clap::Subcommand, Debug)]
#[allow(dead_code)]
enum Command {
    /// Print the comment required to initiate FCP
    FCP { path: PathBuf },

    /// Print the RFC text to stdout
    RFC { path: PathBuf },

    /// Set up a new Call For Proposals (CFP) period
    CFP {
        /// Timeframe for the new CFP period (e.g., 2025h1)
        timeframe: String,

        /// Force overwrite without asking for confirmation
        #[arg(short = 'f', long = "force")]
        force: bool,

        /// Dry run - don't make any changes, just show what would be done
        #[arg(short = 'n', long = "dry-run")]
        dry_run: bool,
    },

    /// Use `gh` CLI tool to create issues on the rust-lang/rust-project-goals repository
    Issues {
        path: PathBuf,

        /// Number of milliseconds to pause between github commands
        /// to avoid rate limiting
        #[arg(long, default_value = "500")]
        sleep: u64,

        /// Without this option, no action is taken.
        #[arg(long)]
        commit: bool,
    },

    /// Generate the project-goal-owners team based on the owners found in `paths`.
    TeamRepo {
        /// Paths to the directories containing the goals (e.g., `src/2024h2`)
        #[arg(required = true /* , min_values = 1  */)]
        path: Vec<PathBuf>,

        /// Paths to the teams repository checkout
        #[arg(required = true, long = "team-repo")]
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
        #[arg(long)]
        json_path: Option<PathBuf>,
    },

    /// Generate markdown with the list of updates for each tracking issue.
    /// Collects goal updates.
    Updates {
        #[command(flatten)]
        updates: UpdateArgs,
    },
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();

    match &opt.cmd {
        Command::FCP { path } => {
            rfc::generate_comment(&path)?;
        }

        Command::CFP {
            timeframe,
            force,
            dry_run,
        } => {
            cfp::create_cfp(timeframe, *force, *dry_run)?;
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
        Command::Updates { updates } => {
            let status = std::process::Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg("rust-project-goals-cli-llm")
                .arg("-q")
                .arg("--")
                .arg(&opt.repository.to_string())
                .arg(&serde_json::to_string(updates).unwrap())
                .status()?;
            if !status.success() {
                bail!("subcommand failed");
            }
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

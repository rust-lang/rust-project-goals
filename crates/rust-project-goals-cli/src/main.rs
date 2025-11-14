use clap::Parser;
use regex::Regex;
use rust_project_goals::{
    gh::issue_id::Repository,
    spanned::{Context as _, Result, Spanned},
};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Stdio};
use walkdir::WalkDir;

mod cfp;
mod csv_reports;
mod rfc;
mod team_repo;
mod updates;

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

    /// Generate markdown with the list of updates for each tracking issue.
    /// Collects goal updates.
    Updates {
        /// Milestone for which we generate tracking issue data (e.g., `2024h2`).
        milestone: String,

        /// Open the generated summary in vscode.
        #[arg(long)]
        vscode: bool,

        /// If specified, write the output into the given file.
        #[arg(long)]
        output_file: Option<PathBuf>,

        /// Start date for comments.
        /// If not given, defaults to 1 week before the start of this month.
        start_date: Option<chrono::NaiveDate>,

        /// End date for comments.
        /// If not given, no end date.
        end_date: Option<chrono::NaiveDate>,

        /// Filter to only include goals that have a champion from the specified team.
        #[arg(long)]
        with_champion_from: Option<String>,
    },

    /// Generate various CSV reports
    CSV {
        #[command(subcommand)]
        cmd: CSVReports,
    },
}

#[derive(clap::Subcommand, Debug)]
#[allow(dead_code)]
enum CSVReports {
    Champions {
        /// Milestone for which we generate tracking issue data (e.g., `2024h2`).
        milestone: String,
    },
}

fn main() -> Result<()> {
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
            rfc::generate_issues(&opt.repository, path, *commit, *sleep).map_err(|e| {
                e.wrap_str(Spanned::here(
                    "failed to adjust issues; rerun command to resume",
                ))
            })?;
        }

        Command::TeamRepo {
            path,
            team_repo_path,
        } => {
            team_repo::generate_team_repo(&path, team_repo_path)?;
        }

        Command::Updates {
            milestone,
            vscode,
            output_file,
            start_date,
            end_date,
            with_champion_from,
        } => generate_updates(
            &opt.repository,
            milestone,
            output_file.as_deref(),
            start_date,
            end_date,
            *vscode,
            with_champion_from.as_deref(),
        )?,

        Command::CSV { cmd } => csv_reports::csv(&opt.repository, cmd)?,
    }

    Ok(())
}

fn check() -> Result<()> {
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

fn generate_updates(
    repository: &Repository,
    milestone: &str,
    output_file: Option<&Path>,
    start_date: &Option<chrono::NaiveDate>,
    end_date: &Option<chrono::NaiveDate>,
    vscode: bool,
    with_champion_from: Option<&str>,
) -> Result<()> {
    if output_file.is_none() && !vscode {
        rust_project_goals::spanned::bail_here!(
            "either `--output-file` or `--vscode` must be specified"
        );
    }

    // Load milestone issues first (Step 2: Update CLI to use new API)
    let issues = rust_project_goals::gh::issues::list_issues_in_milestone(repository, milestone)?;

    // Generate the updates content using the library function with progress bar
    let output = updates::render_updates(
        &issues,
        repository,
        milestone,
        start_date,
        end_date,
        with_champion_from,
        true,
	updates::Order::default(),
    )?;

    if let Some(output_file) = output_file {
        std::fs::write(&output_file, output).with_path_context(output_file, "failed to write")?;
    } else if vscode {
        let mut child = ProcessCommand::new("code")
            .arg("-")
            .stdin(Stdio::piped())
            .spawn()
            .with_str_context("failed to spawn `code` process")?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(output.as_bytes())
                .with_str_context("failed to write to `code` stdin")?;
        }

        child
            .wait()
            .with_str_context("failed to wait on `code` process")?;
    } else {
        println!("{output}");
    }

    Ok(())
}

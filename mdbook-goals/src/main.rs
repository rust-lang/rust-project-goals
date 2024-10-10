use anyhow::Context;
use gh::issue_id::Repository;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_preprocessor::GoalPreprocessor;
use regex::Regex;
use semver::{Version, VersionReq};
use std::{io, path::PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

mod gh;
mod goal;
mod json;
mod markwaydown;
mod mdbook_preprocessor;
mod re;
mod rfc;
mod team;
mod team_repo;
mod updates;
mod util;

#[derive(StructOpt, Debug)]
#[structopt(about = "Project goal preprocessor")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,

    /// Repository to use if applicable
    #[structopt(long, default_value = "rust-lang/rust-project-goals")]
    repository: Repository,
}

#[derive(StructOpt, Debug)]
#[allow(dead_code)]
enum Command {
    /// Command used by mdbook to check if the preprocessor supports a renderer
    Supports { renderer: String },

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

        /// Start date for comments.
        /// If not given, defaults to 1 week before the start of this month.
        start_date: Option<chrono::NaiveDate>,

        /// End date for comments.
        /// If not given, no end date.
        end_date: Option<chrono::NaiveDate>,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let Some(cmd) = &opt.cmd else {
        return handle_preprocessing(&GoalPreprocessor);
    };

    match cmd {
        Command::Supports { renderer } => {
            handle_supports(&GoalPreprocessor, renderer)?;
        }

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
            json::generate_json(&opt.repository, &milestone, json_path)?;
        }
        Command::Updates {
            milestone,
            start_date,
            end_date,
        } => {
            updates::updates(&opt.repository, milestone, start_date, end_date)?;
        }
    }

    Ok(())
}

// from https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs
fn handle_supports(pre: &dyn Preprocessor, renderer: &str) -> anyhow::Result<()> {
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        Ok(())
    } else {
        anyhow::bail!("renderer `{}` unsupported", renderer)
    }
}

// from https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs
fn handle_preprocessing(pre: &dyn Preprocessor) -> anyhow::Result<()> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

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

        let _goals = goal::goals_in_dir(entry.path())?;
    }

    Ok(())
}

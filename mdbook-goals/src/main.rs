use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_preprocessor::GoalPreprocessor;
use regex::Regex;
use semver::{Version, VersionReq};
use std::{io, path::PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

mod goal;
mod markwaydown;
mod mdbook_preprocessor;
mod re;
mod rfc;
mod team;
mod util;

#[derive(StructOpt, Debug)]
#[structopt(about = "Project goal preprocessor")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,

    /// Repository to use if applicable
    #[structopt(long, default_value = "rust-lang/rust-project-goals")]
    repository: String,
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

        /// Without this option, no action is taken.
        #[structopt(long)]
        commit: bool,
    },

    /// Checks that the goal documents are well-formed, intended for use within CI
    Check {},
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match &opt.cmd {
        Some(Command::Supports { renderer }) => {
            handle_supports(&GoalPreprocessor, renderer)?;
        }

        Some(Command::FCP { path }) => {
            rfc::generate_comment(&path)?;
        }

        Some(Command::Check {}) => {
            check()?;
        }

        Some(Command::RFC { path }) => {
            rfc::generate_rfc(&path)?;
        }

        Some(Command::Issues { path, commit }) => {
            rfc::generate_issues(&opt.repository, path, *commit)?;
        }

        None => {
            handle_preprocessing(&GoalPreprocessor)?;
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

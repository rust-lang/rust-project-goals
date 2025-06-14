//! Library for generating updates -- just encodes the command-line arguments.
//! Most of the work is in the `main.rs` binary.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Updates struct
#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct UpdateArgs {
    /// Milestone for which we generate tracking issue data (e.g., `2024h2`).
    pub milestone: String,

    /// Open the generated summary in vscode.
    #[arg(long)]
    pub vscode: bool,

    /// If specified, write the output into the given file.
    #[arg(long)]
    pub output_file: Option<PathBuf>,

    /// Start date for comments.
    /// If not given, defaults to 1 week before the start of this month.
    pub start_date: Option<chrono::NaiveDate>,

    /// End date for comments.
    /// If not given, no end date.
    pub end_date: Option<chrono::NaiveDate>,
}

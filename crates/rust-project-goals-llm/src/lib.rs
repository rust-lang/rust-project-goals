//! Library for the LLM execution -- just encodes the command-line arguments.
//! Most of the work is in the `main.rs` binary.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Updates struct
#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct UpdateArgs {
    /// Milestone for which we generate tracking issue data (e.g., `2024h2`).
    pub milestone: String,

    /// Quick mode does not use an LLM to generate a summary.
    #[arg(long)]
    pub quick: bool,

    /// Quick mode does not use an LLM to generate a summary.
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

    /// Set a custom model id for the LLM.
    #[arg(long)]
    pub model_id: Option<String>,

    /// Set a custom region.
    #[arg(long)]
    pub region: Option<String>,
}

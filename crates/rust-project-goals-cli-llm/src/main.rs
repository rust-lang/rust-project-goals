//! Code to collect updates on tracking issues and generate blog posts.

use clap::Parser;
use rust_project_goals::gh::issue_id::Repository;
use rust_project_goals_llm::UpdateArgs;

mod templates;
mod updates;

#[derive(clap::Parser, Debug)]
#[structopt(about = "Project goal preprocessor")]
struct Opt {
    repository: Repository,
    updates_json: String,
}

fn main() -> anyhow::Result<()> {
    let Opt {
        repository,
        updates_json,
    } = Opt::parse();
    let UpdateArgs {
        milestone,
        vscode,
        output_file,
        start_date,
        end_date,
    } = &serde_json::from_str(&updates_json)?;
    updates::generate_updates(
        &repository,
        milestone,
        output_file.as_deref(),
        start_date,
        end_date,
        *vscode,
    )
}

//! Code to invoke a LLM to summarize content and generate blog posts.
//! Currently based on AWS bedrock.

use clap::Parser;
use rust_project_goals::gh::issue_id::Repository;
use rust_project_goals_llm::UpdateArgs;

mod llm;
mod templates;
mod updates;

#[derive(clap::Parser, Debug)]
#[structopt(about = "Project goal preprocessor")]
struct Opt {
    repository: Repository,    
    updates_json: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Opt { repository, updates_json } = Opt::parse();
    let UpdateArgs { milestone, quick, vscode, output_file, start_date, end_date, model_id, region } = 
    &serde_json::from_str(&updates_json)?;
    updates::updates(&repository, milestone, output_file.as_deref(), start_date, end_date, *quick, *vscode, model_id.as_deref(), region.as_deref()).await?;
    Ok(())
}
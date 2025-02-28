use std::path::PathBuf;

use anyhow::Context;
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    /// Defines the valid "asks" of teams. The key is the ask, the value is an extended description.
    /// IndexMap is used to preserve the ordering as defined in the TOML file.
    pub team_asks: IndexMap<String, TeamAskDetails>,
}

#[derive(Deserialize)]
pub struct TeamAskDetails {
    /// A short descriptor of the team ask suitable for inclusion in a table
    pub short: String,

    /// Longer description
    pub about: String,
}

impl Configuration {
    pub fn get() -> &'static Configuration {
        lazy_static::lazy_static! {
            static ref CONFIG: Configuration = Configuration::load().unwrap();
        }
        &*CONFIG
    }

    fn load() -> anyhow::Result<Configuration> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let toml_file = manifest_dir.join("../../rust-project-goals.toml");
        let toml_string = std::fs::read_to_string(&toml_file)
            .with_context(|| format!("loading configuration from {}", toml_file.display()))?;
        Ok(toml::from_str(&toml_string)?)
    }
}

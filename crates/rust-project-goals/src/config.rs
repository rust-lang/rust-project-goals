use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub team_asks: BTreeMap<String, String>,
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
        let toml_string = std::fs::read_to_string(&toml_file).with_context(|| format!("loading configuration from {}", toml_file.display()))?;
        Ok(toml::from_str(&toml_string)?)
    }
}

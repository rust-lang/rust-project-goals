use std::{collections::BTreeMap, sync::OnceLock};

use rust_team_data::v1;
use serde::de::DeserializeOwned;

trait Load<T> {
    fn load(&self, op: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<&T>;
}

impl<T> Load<T> for OnceLock<anyhow::Result<T>> {
    fn load(&self, op: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<&T> {
        match self.get_or_init(op) {
            Ok(data) => Ok(data),
            Err(e) => Err(anyhow::anyhow!("failed to fetch: {e:?}")),
        }
    }
}

/// Given a username like `@foo` finds the corresponding person data (if any).
pub fn get_person_data(username: &str) -> anyhow::Result<Option<&'static v1::Person>> {
    static DATA: OnceLock<anyhow::Result<BTreeMap<String, v1::Person>>> = OnceLock::new();
    let people = DATA.load(|| {
        let data: v1::People = fetch("people.json")?;
        Ok(data
            .people
            .into_iter()
            .map(|(username, value)| (username.to_lowercase(), value))
            .collect())
    })?;

    Ok(people.get(&username[1..].to_lowercase()))
}

// pub fn get_teams() -> anyhow::Result<&'static v1::Teams> {
//     static DATA: OnceLock<anyhow::Result<v1::Teams>> = OnceLock::new();
//     DATA.load(|| fetch("teams.json"))
// }

fn fetch<T>(path: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let url = format!("{}/{}", v1::BASE_URL, path);
    Ok(reqwest::blocking::get(&url)?.json()?)
}

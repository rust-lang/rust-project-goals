use std::sync::OnceLock;

use rust_team_data::v1;
use serde::de::DeserializeOwned;

pub trait RustTeamData {
    type Data: DeserializeOwned;
    const PATH: &'static str;
    fn data() -> &'static OnceLock<anyhow::Result<Self::Data>>;

    fn get() -> anyhow::Result<&'static Self::Data> {
        match Self::data().get_or_init(|| fetch(Self::PATH)) {
            Ok(data) => Ok(data),
            Err(e) => Err(anyhow::anyhow!("failed to fetch: {e:?}")),
        }
    }
}

pub struct People;
impl RustTeamData for People {
    type Data = v1::People;
    const PATH: &'static str = "people.json";
    fn data() -> &'static OnceLock<anyhow::Result<Self::Data>> {
        static S: OnceLock<anyhow::Result<v1::People>> = OnceLock::new();
        &S
    }
}

pub struct Teams;
impl RustTeamData for Teams {
    type Data = v1::Teams;
    const PATH: &'static str = "teams.json";
    fn data() -> &'static OnceLock<anyhow::Result<Self::Data>> {
        static S: OnceLock<anyhow::Result<v1::Teams>> = OnceLock::new();
        &S
    }
}

fn fetch<T>(path: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let url = format!("{}/{}", v1::BASE_URL, path);
    Ok(reqwest::blocking::get(&url)?.json()?)
}

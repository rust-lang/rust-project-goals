use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TEAM_ASKS: Regex = Regex::new(r"<!-- TEAM ASKS -->").unwrap();
}

lazy_static! {
    pub static ref GOAL_LIST: Regex = Regex::new(r"<!-- GOALS '(.*)' -->").unwrap();
}

lazy_static! {
    pub static ref GOAL_COUNT: Regex = Regex::new(r"<!-- #GOALS -->").unwrap();
}

lazy_static! {
    pub static ref USERNAME: Regex = Regex::new(r"@([-a-zA-Z0-9])+").unwrap();
}

lazy_static! {
    pub static ref TRACKING_ISSUE: Regex = Regex::new(r"\[([^#]*)#([0-9]+)\]").unwrap();
}

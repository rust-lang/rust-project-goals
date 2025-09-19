use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TEAM_ASKS: Regex = Regex::new(r"\(\(\(TEAM ASKS\)\)\)").unwrap();
}

lazy_static! {
    pub static ref CHAMPIONS: Regex = Regex::new(r"\(\(\(CHAMPIONS\)\)\)").unwrap();
}

// List of all goals, flagship or otherwise
lazy_static! {
    pub static ref GOAL_LIST: Regex = Regex::new(r"\(\(\(GOALS\)\)\)").unwrap();
}

// List of flagship goals (accepted or pending)
lazy_static! {
    pub static ref FLAGSHIP_GOAL_LIST: Regex = Regex::new(r"\(\(\(FLAGSHIP GOALS\)\)\)").unwrap();
}

// List of flagship goals filtered by category (accepted or pending)
lazy_static! {
    pub static ref FLAGSHIP_GOAL_LIST_FILTERED: Regex =
        Regex::new(r"\(\(\(FLAGSHIP GOALS:\s*(.+?)\s*\)\)\)").unwrap();
}

// List of non-flagship goals (accepted or pending)
lazy_static! {
    pub static ref OTHER_GOAL_LIST: Regex = Regex::new(r"\(\(\(OTHER GOALS\)\)\)").unwrap();
}

// List of not accepted goals
lazy_static! {
    pub static ref GOAL_NOT_ACCEPTED_LIST: Regex =
        Regex::new(r"\(\(\(GOALS NOT ACCEPTED\)\)\)").unwrap();
}

lazy_static! {
    pub static ref GOAL_COUNT: Regex = Regex::new(r"\(\(\(#GOALS\)\)\)").unwrap();
}

lazy_static! {
    pub static ref FLAGSHIP_GOAL_COUNT: Regex = Regex::new(r"\(\(\(#FLAGSHIP GOALS\)\)\)").unwrap();
}

lazy_static! {
    pub static ref VALID_TEAM_ASKS: Regex = Regex::new(r"\(\(\(VALID TEAM ASKS\)\)\)").unwrap();
}

lazy_static! {
    /// Github username.
    ///
    /// According to [this random page I found with a google search](https://github.com/GrantBirki/github-username-regex-js):
    /// * GitHub usernames may only contain alphanumeric characters or hyphens
    /// * GitHub usernames cannot have multiple consecutive hyphens
    /// * GitHub usernames cannot begin or end with a hyphen
    /// * Usernames can have a maximum of 39 characters
    pub static ref USERNAME: Regex = Regex::new(r"@([-_a-zA-Z0-9])+").unwrap();
}

lazy_static! {
    pub static ref REPOSITORY: Regex = Regex::new(r"([^#/]*)/([^#/]*)").unwrap();
}

lazy_static! {
    pub static ref TRACKING_ISSUE: Regex = Regex::new(r"\[([^#/]*)/([^#/]*)#([0-9]+)\]").unwrap();
}

lazy_static! {
    pub static ref GITHUB_ISSUE_URL: Regex =
        Regex::new(r"https://github.com/([^#/]*)/([^#/]*)/issues/([0-9]+)").unwrap();
}

lazy_static! {
    pub static ref CHECKBOX: Regex = Regex::new(r"\s*[-*] \[[ x]\] ").unwrap();
}

lazy_static! {
    pub static ref CHECKED_CHECKBOX: Regex = Regex::new(r"\s*[-*] \[x\] ").unwrap();
}

lazy_static! {
    pub static ref TRACKED_ISSUES_QUERY: Regex =
        Regex::new(r"^\| *Tracked +issues *\| *\[(?P<repo>[^ ]*) (?P<query>[^]]*)\]\(.*\) *\| *$")
            .unwrap();
}

lazy_static! {
    pub static ref SEE_ALSO_QUERY: Regex =
        Regex::new(r"^\| *See also *\|(?P<issues>[^|]+)\| *$").unwrap();
}

lazy_static! {
    pub static ref SEE_ALSO_ISSUE1: Regex =
        Regex::new(r"(?P<org>[^#/]*)/(?P<repo>[^#/]*)#(?P<issue>[0-9]+)").unwrap();
}

lazy_static! {
    pub static ref SEE_ALSO_ISSUE2: Regex =
        Regex::new(r"https://github.com/(?P<org>[^#/]*)/(?P<repo>[^#/]*)/issues/(?P<issue>[0-9]+)")
            .unwrap();
}

/// True if the entire string `s` matches `re`
pub fn is_just(re: &Regex, s: &str) -> bool {
    let output = re.replace(s, "X");
    output == "X"
}

lazy_static! {
    /// If a line within a comment begins with this text, it will be considered a request for help
    pub static ref HELP_WANTED: Regex =
        Regex::new(r"^[*-]?\s*(?i:help wanted:|\*\*help wanted:\*\*) (?P<text>.*)")
            .unwrap();
}

/// If a comment begins with this text, it will be considered a summary.
pub const TLDR: &str = "TL;DR:";

lazy_static! {
    /// Metadata table rows like `[lang] champion` indicate the champion for the lang team
    pub static ref CHAMPION_METADATA: Regex =
        Regex::new(r"^\s*\[(?P<team>.*)\] champion\s*$")
            .unwrap();
}

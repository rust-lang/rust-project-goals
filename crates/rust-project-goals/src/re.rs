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

// List of large goals (goals with at least one Large team ask)
lazy_static! {
    pub static ref LARGE_GOAL_LIST: Regex = Regex::new(r"\(\(\(LARGE GOALS\)\)\)").unwrap();
}

// List of medium goals (goals with at least one Medium ask, no Large asks)
lazy_static! {
    pub static ref MEDIUM_GOAL_LIST: Regex = Regex::new(r"\(\(\(MEDIUM GOALS\)\)\)").unwrap();
}

// List of small goals (goals with only Small or Vibes asks)
lazy_static! {
    pub static ref SMALL_GOAL_LIST: Regex = Regex::new(r"\(\(\(SMALL GOALS\)\)\)").unwrap();
}

// Marker to create goal subchapters without rendering a table
lazy_static! {
    pub static ref GOAL_CHAPTERS: Regex = Regex::new(r"\(\(\(GOAL CHAPTERS\)\)\)").unwrap();
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
    /// GitHub username.
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

lazy_static! {
    /// Reports placeholder with optional date range
    pub static ref REPORTS: Regex =
        Regex::new(r"\(\(\(REPORTS(?::\s*([^)]+))?\)\)\)")
            .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reports_regex_start_and_end_date() {
        assert!(REPORTS.is_match("(((REPORTS)))"));
        assert!(REPORTS.is_match("(((REPORTS: 2025-09-01 to 2025-12-31)))"));

        let caps = REPORTS
            .captures("(((REPORTS: 2025-09-01 to 2025-12-31)))")
            .unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "2025-09-01 to 2025-12-31");
    }

    #[test]
    fn test_reports_regex_no_end_date() {
        assert!(REPORTS.is_match("(((REPORTS)))"));
        assert!(REPORTS.is_match("(((REPORTS: 2025-09-01)))"));

        let caps = REPORTS.captures("(((REPORTS: 2025-09-01)))").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "2025-09-01");
    }
}

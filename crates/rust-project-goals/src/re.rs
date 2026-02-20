use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TEAM_ASKS: Regex = Regex::new(r"\(\(\(TEAM ASKS\)\)\)").unwrap();
}

lazy_static! {
    pub static ref CHAMPIONS: Regex = Regex::new(r"\(\(\(CHAMPIONS\)\)\)").unwrap();
}

// List of all goals, roadmap or otherwise
lazy_static! {
    pub static ref GOALS_LIST: Regex = Regex::new(r"\(\(\(GOALS\)\)\)").unwrap();
}

// List of roadmap goals (accepted or pending)
// Accepts both "ROADMAP GOALS" (new) and "FLAGSHIP GOALS" (old) for backward compatibility
lazy_static! {
    pub static ref ROADMAP_GOALS_LIST: Regex = Regex::new(r"\(\(\((?:ROADMAP|FLAGSHIP) GOALS\)\)\)").unwrap();
}

// List of roadmap goals filtered by category (accepted or pending)
// Accepts both "ROADMAP GOALS:" (new) and "FLAGSHIP GOALS:" (old) for backward compatibility
lazy_static! {
    pub static ref ROADMAP_GOALS_LIST_FILTERED: Regex =
        Regex::new(r"\(\(\((?:ROADMAP|FLAGSHIP) GOALS:\s*(.+?)\s*\)\)\)").unwrap();
}

// List of non-roadmap goals (accepted or pending)
lazy_static! {
    pub static ref OTHER_GOALS_LIST: Regex = Regex::new(r"\(\(\(OTHER GOALS\)\)\)").unwrap();
}

// List of not accepted goals
lazy_static! {
    pub static ref GOALS_NOT_ACCEPTED_LIST: Regex =
        Regex::new(r"\(\(\(GOALS NOT ACCEPTED\)\)\)").unwrap();
}

// List of large goals (goals with at least one Large team ask)
lazy_static! {
    pub static ref LARGE_GOALS_LIST: Regex = Regex::new(r"\(\(\(LARGE GOALS\)\)\)").unwrap();
}

// List of medium goals (goals with at least one Medium ask, no Large asks)
lazy_static! {
    pub static ref MEDIUM_GOALS_LIST: Regex = Regex::new(r"\(\(\(MEDIUM GOALS\)\)\)").unwrap();
}

// List of small goals (goals with only Small asks)
lazy_static! {
    pub static ref SMALL_GOALS_LIST: Regex = Regex::new(r"\(\(\(SMALL GOALS\)\)\)").unwrap();
}

// Marker to create goal subchapters without rendering a table
lazy_static! {
    pub static ref GOAL_CHAPTERS: Regex = Regex::new(r"\(\(\(GOAL CHAPTERS\)\)\)").unwrap();
}

// List of highlight goals filtered by theme (accepted or pending)
lazy_static! {
    pub static ref HIGHLIGHT_GOALS_LIST_FILTERED: Regex =
        Regex::new(r"\(\(\(HIGHLIGHT GOALS:\s*(.+?)\s*\)\)\)").unwrap();
}

// List of goals with needs filtered by need (e.g. Funding, Contributor)
lazy_static! {
    pub static ref GOALS_WITH_NEEDS_LIST_FILTERED: Regex =
        Regex::new(r"\(\(\(GOALS WITH NEEDS:\s*(.+?)\s*\)\)\)").unwrap();
}

lazy_static! {
    pub static ref GOALS_COUNT: Regex = Regex::new(r"\(\(\(#GOALS\)\)\)").unwrap();
}

// Accepts both "#ROADMAP GOALS" (new) and "#FLAGSHIP GOALS" (old) for backward compatibility
lazy_static! {
    pub static ref ROADMAP_GOALS_COUNT: Regex = Regex::new(r"\(\(\(#(?:ROADMAP|FLAGSHIP) GOALS\)\)\)").unwrap();
}

lazy_static! {
    pub static ref VALID_TEAM_ASKS: Regex = Regex::new(r"\(\(\(VALID TEAM ASKS\)\)\)").unwrap();
}

// List of all roadmap documents (the roadmap-*.md files themselves, not goals tagged with a roadmap)
lazy_static! {
    pub static ref ROADMAPS: Regex = Regex::new(r"\(\(\(ROADMAPS\)\)\)").unwrap();
}

// Marker to create roadmap subchapters without rendering a table
lazy_static! {
    pub static ref ROADMAP_CHAPTERS: Regex = Regex::new(r"\(\(\(ROADMAP CHAPTERS\)\)\)").unwrap();
}
lazy_static! {
    pub static ref ROADMAPS_FILTERED: Regex =
        Regex::new(r"\(\(\(ROADMAPS:\s*(.+?)\s*\)\)\)").unwrap();
}

// Table of application areas and their associated roadmaps
lazy_static! {
    pub static ref APPLICATION_AREAS: Regex =
        Regex::new(r"\(\(\(APPLICATION AREAS\)\)\)").unwrap();
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
    /// Matches a markdown link like `[link text](url)` and captures the link text.
    pub static ref MARKDOWN_LINK: Regex =
        Regex::new(r"^\[(?P<text>[^\]]+)\]\([^)]+\)$")
            .unwrap();
}

/// If `s` is a markdown link like `[text](url)`, return the link text.
/// Otherwise return the original string (trimmed).
pub fn strip_markdown_link(s: &str) -> &str {
    let trimmed = s.trim();
    if let Some(caps) = MARKDOWN_LINK.captures(trimmed) {
        caps.name("text").unwrap().as_str()
    } else {
        trimmed
    }
}

lazy_static! {
    /// Reports placeholder with optional date range
    pub static ref REPORTS: Regex =
        Regex::new(r"\(\(\(REPORTS(?::\s*([^)]+))?\)\)\)")
            .unwrap();
}

lazy_static! {
    pub static ref IDENTIFIERS: Regex = Regex::new(r"[-.A-Za-z]+").unwrap();
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

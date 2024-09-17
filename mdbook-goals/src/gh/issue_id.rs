use crate::re::TRACKING_ISSUE;
use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IssueId {
    /// Something like `rust-lang/rust-project-goals`
    pub repository: String,

    /// Something like `22`
    pub number: u64,
}

impl IssueId {
    pub fn new(repository: &(impl Display + ?Sized), number: u64) -> Self {
        Self {
            repository: repository.to_string(),
            number,
        }
    }
}

impl std::fmt::Debug for IssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for IssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{repository}#{number}]",
            repository = self.repository,
            number = self.number,
        )
    }
}

impl std::str::FromStr for IssueId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(c) = TRACKING_ISSUE.captures(s) else {
            anyhow::bail!("invalid issue-id")
        };

        Ok(IssueId::new(&c[1], c[2].parse()?))
    }
}

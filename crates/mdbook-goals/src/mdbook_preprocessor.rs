use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use anyhow::Context;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::{Captures, Regex};
use rust_project_goals::config::Configuration;
use rust_project_goals::format_team_ask::format_team_asks;
use rust_project_goals::util::{self, GithubUserInfo};

use rust_project_goals::{
    goal::{self, GoalDocument, Status, TeamAsk},
    re, team,
};
use spanned::Spanned;

const LINKS: &str = "links";
const LINKIFIERS: &str = "linkifiers";
const USERS: &str = "users";
const IGNORE_USERS: &str = "ignore_users";

pub struct GoalPreprocessor;

impl Preprocessor for GoalPreprocessor {
    fn name(&self) -> &str {
        "goals"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        eprintln!("running goal preprocessor");
        let mut this = GoalPreprocessorWithContext::new(ctx)?;
        eprintln!("processing book items");
        for section in &mut book.sections {
            eprintln!("book item: {:?}", section);
            this.process_book_item(section)?;
        }
        eprintln!("book items done");
        Ok(book)
    }
}

pub struct GoalPreprocessorWithContext<'c> {
    team_asks: &'static Regex,
    goal_count: &'static Regex,
    username: &'static Regex,
    ctx: &'c PreprocessorContext,
    links: Vec<(String, String)>,
    linkifiers: Vec<(Regex, String)>,
    display_names: BTreeMap<String, Rc<String>>,
    ignore_users: Vec<String>,
    goal_document_map: BTreeMap<PathBuf, Arc<Vec<GoalDocument>>>,
}

impl<'c> GoalPreprocessorWithContext<'c> {
    pub fn new(ctx: &'c PreprocessorContext) -> anyhow::Result<Self> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        let mut links: Vec<(String, String)> = Default::default();
        let mut linkifiers = Default::default();
        let mut display_names: BTreeMap<String, Rc<String>> = Default::default();
        let mut ignore_users: Vec<String> = Default::default();
        if let Some(config) = ctx.config.get_preprocessor(GoalPreprocessor.name()) {
            if let Some(value) = config.get(LINKS) {
                links = value
                    .as_table()
                    .with_context(|| format!("`{}` must be a table", LINKS))?
                    .iter()
                    .map(|(k, v)| {
                        if let Some(v) = v.as_str() {
                            Ok((k.to_string(), v.to_string()))
                        } else {
                            Err(anyhow::anyhow!("link value `{}` must be a string", k))
                        }
                    })
                    .collect::<Result<_, _>>()?;
            }

            if let Some(value) = config.get(LINKIFIERS) {
                linkifiers = value
                    .as_table()
                    .with_context(|| format!("`{}` must be a table", LINKIFIERS))?
                    .iter()
                    .map(|(k, v)| {
                        if let Some(v) = v.as_str() {
                            Ok((Regex::new(&format!(r"\[{}\]", k))?, v.to_string()))
                        } else {
                            Err(anyhow::anyhow!(
                                "linkifier value for `{}` must be a string",
                                k
                            ))
                        }
                    })
                    .collect::<Result<_, _>>()?;
            }

            if let Some(value) = config.get(USERS) {
                let users = value
                    .as_table()
                    .with_context(|| format!("`{}` must be a table", USERS))?
                    .iter()
                    .map(|(k, v)| {
                        if !k.starts_with("@") {
                            Err(anyhow::anyhow!("user name `{k}` does not start with `@`"))
                        } else if let Some(v) = v.as_str() {
                            Ok((k.to_string(), v.to_string()))
                        } else {
                            Err(anyhow::anyhow!(
                                "display name for user `{k}` must be a string",
                            ))
                        }
                    });

                for user in users {
                    let (user, display_name) = user?;

                    display_names.insert(user, Rc::new(display_name));
                }
            }

            if let Some(value) = config.get(IGNORE_USERS) {
                ignore_users = value
                    .as_array()
                    .with_context(|| format!("`{}` must be an array", IGNORE_USERS))?
                    .iter()
                    .map(|v| {
                        if let Some(v) = v.as_str() {
                            Ok(v.to_string())
                        } else {
                            Err(anyhow::anyhow!(
                                "ignore user value `{}` must be a string",
                                v
                            ))
                        }
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;
            }
        }

        Ok(GoalPreprocessorWithContext {
            ctx,
            team_asks: &re::TEAM_ASKS,
            goal_count: &re::GOAL_COUNT,
            username: &re::USERNAME,
            links,
            linkifiers,
            display_names,
            ignore_users,
            goal_document_map: Default::default(),
        })
    }

    fn process_book_item(&mut self, book_item: &mut BookItem) -> anyhow::Result<()> {
        match book_item {
            BookItem::Chapter(chapter) => {
                self.replace_metadata_placeholders(chapter)?;
                self.replace_team_asks(chapter)?;
                self.replace_valid_team_asks(chapter)?;
                self.replace_goal_lists(chapter)?;
                self.replace_goal_count(chapter)?;
                self.link_teams(chapter)?;
                self.link_users(chapter)?;
                self.linkify(chapter)?;
                self.insert_links(chapter)?;

                for sub_item in &mut chapter.sub_items {
                    self.process_book_item(sub_item)?;
                }

                Ok(())
            }

            BookItem::Separator => Ok(()),

            BookItem::PartTitle(_) => Ok(()),
        }
    }

    fn replace_goal_count(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        if !self.goal_count.is_match(&chapter.content) {
            return Ok(());
        }

        let Some(chapter_path) = &chapter.path else {
            anyhow::bail!("found `<!-- #GOALS -->` but chapter has no path")
        };

        let goals = self.goal_documents(chapter_path)?;

        let count = goals
            .iter()
            .filter(|g| g.metadata.status.is_not_not_accepted())
            .count();

        chapter.content = self
            .goal_count
            .replace_all(&chapter.content, &count.to_string())
            .to_string();

        Ok(())
    }

    fn replace_goal_lists(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        self.replace_goal_lists_helper(chapter, &re::FLAGSHIP_GOAL_LIST, |status| {
            status.is_flagship && status.is_not_not_accepted()
        })?;
        self.replace_goal_lists_helper(chapter, &re::OTHER_GOAL_LIST, |status| {
            !status.is_flagship && status.is_not_not_accepted()
        })?;
        self.replace_goal_lists_helper(chapter, &re::GOAL_LIST, |status| {
            status.is_not_not_accepted()
        })?;
        self.replace_goal_lists_helper(chapter, &re::GOAL_NOT_ACCEPTED_LIST, |status| {
            !status.is_not_not_accepted()
        })?;
        Ok(())
    }

    fn replace_goal_lists_helper(
        &mut self,
        chapter: &mut Chapter,
        regex: &Regex,
        filter: impl Fn(Status) -> bool,
    ) -> anyhow::Result<()> {
        loop {
            let Some(m) = regex.find(&chapter.content) else {
                return Ok(());
            };
            let range = m.range();

            let Some(chapter_path) = &chapter.path else {
                anyhow::bail!("found `{regex}` but chapter has no path")
            };

            // Extract out the list of goals with the given status.
            let goals = self.goal_documents(chapter_path)?;
            let mut goals_with_status: Vec<&GoalDocument> =
                goals.iter().filter(|g| filter(g.metadata.status)).collect();

            goals_with_status.sort_by_key(|g| &g.metadata.title);

            // Format the list of goals and replace the `<!-- -->` comment with that.
            let output = goal::format_goal_table(&goals_with_status)?;
            chapter.content.replace_range(range, &output);

            // Populate with children if this is not README
            if chapter_path.file_stem() != Some("README".as_ref()) {
                let mut parent_names = chapter.parent_names.clone();
                parent_names.push(chapter.name.clone());
                for (goal, index) in goals_with_status.iter().zip(0..) {
                    let content = std::fs::read_to_string(&goal.path)
                        .with_context(|| format!("reading `{}`", goal.path.display()))?;
                    let path = goal.path.strip_prefix(&self.ctx.config.book.src).unwrap();
                    let mut new_chapter =
                        Chapter::new(&goal.metadata.title, content, path, parent_names.clone());

                    if let Some(mut number) = chapter.number.clone() {
                        number.0.push(index + 1);
                        new_chapter.number = Some(number);
                    }

                    chapter.sub_items.push(BookItem::Chapter(new_chapter));
                }
            }
        }
    }

    /// Look for `<!-- TEAM ASKS -->` in the chapter content and replace it with the team asks.
    fn replace_team_asks(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let Some(m) = self.team_asks.find(&chapter.content) else {
            return Ok(());
        };
        let range = m.range();

        let Some(path) = &chapter.path else {
            anyhow::bail!("found `<!-- TEAM ASKS -->` but chapter has no path")
        };

        let goals = self.goal_documents(path)?;
        let asks_of_any_team: Vec<&TeamAsk> = goals
            .iter()
            .filter(|g| g.metadata.status.is_not_not_accepted())
            .flat_map(|g| &g.team_asks)
            .collect();
        let format_team_asks = format_team_asks(&asks_of_any_team)?;
        chapter.content.replace_range(range, &format_team_asks);

        Ok(())
    }

    /// Look for `<!-- TEAM ASKS -->` in the chapter content and replace it with the team asks.
    fn replace_valid_team_asks(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        if !re::VALID_TEAM_ASKS.is_match(&chapter.content) {
            return Ok(());
        }
        let config = Configuration::get();
        let rows = std::iter::once(vec![
            Spanned::here("Ask".to_string()),
            Spanned::here("aka".to_string()),
            Spanned::here("Description".to_string()),
        ])
        .chain(config.team_asks.iter().map(|(name, details)| {
            vec![
                Spanned::here(format!("{name:?}")),
                Spanned::here(details.short.to_string()),
                Spanned::here(details.about.to_string()),
            ]
        }))
        .collect::<Vec<Vec<Spanned<String>>>>();
        let table = util::format_table(&rows);
        let new_content = re::VALID_TEAM_ASKS.replace_all(&chapter.content, table);
        chapter.content = new_content.to_string();
        Ok(())
    }

    /// Find the goal documents for the milestone in which this `chapter_path` resides.
    /// e.g., if invoked with `2024h2/xxx.md`, will find all goal documents in `2024h2`.
    fn goal_documents(&mut self, chapter_path: &Path) -> anyhow::Result<Arc<Vec<GoalDocument>>> {
        let Some(milestone_path) = chapter_path.parent() else {
            anyhow::bail!("cannot get goal documents from `{chapter_path:?}`")
        };

        if let Some(goals) = self.goal_document_map.get(milestone_path) {
            return Ok(goals.clone());
        }

        let goal_documents = goal::goals_in_dir(&self.ctx.config.book.src.join(milestone_path))?;
        let goals = Arc::new(goal_documents);
        self.goal_document_map
            .insert(milestone_path.to_path_buf(), goals.clone());
        Ok(goals)
    }

    fn link_users(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let usernames: BTreeSet<String> = self
            .username
            .find_iter(&chapter.content)
            .map(|m| m.as_str().to_string())
            .filter(|username| !self.ignore_users.contains(username))
            .collect();

        for username in &usernames {
            chapter.content = chapter
                .content
                .replace(username, &format!("[{}][]", self.display_name(username)?));
        }

        chapter.content.push_str("\n\n");
        for username in &usernames {
            chapter.content.push_str(&format!(
                "[{}]: https://github.com/{}\n",
                self.display_name(username)?,
                &username[1..]
            ));
        }

        Ok(())
    }

    fn insert_links(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        chapter.content.push_str("\n\n");

        for (name, url) in &self.links {
            chapter.content.push_str(&format!("[{}]: {}\n", name, url));
        }

        Ok(())
    }

    /// Given a username like `@foo`, determine the "display name" we should use.
    fn display_name<'a>(&mut self, username: &str) -> anyhow::Result<Rc<String>> {
        // Check (in order of priority)...
        //
        // 1. Our cache (pre-populated from the book.toml file)
        // 2. The name from the Rust teams repo
        // 3. The name from the GitHub API (if available)
        //
        // ...and fallback to just `@foo`.

        if let Some(n) = self.display_names.get(username) {
            return Ok(n.clone());
        }

        let display_name = match team::get_person_data(username)? {
            Some(person) => person.data.name.clone(),
            None => match GithubUserInfo::load(username)
                .with_context(|| format!("loading user info for {}", username))
            {
                Ok(GithubUserInfo { name: Some(n), .. }) => n,
                Ok(GithubUserInfo { name: None, .. }) => username.to_string(),
                Err(e) => {
                    eprintln!("{:?}", e);
                    username.to_string()
                }
            },
        };
        let display_name = Rc::new(display_name);
        self.display_names
            .insert(username.to_string(), display_name.clone());
        Ok(display_name)
    }

    fn linkify(&self, chapter: &mut Chapter) -> anyhow::Result<()> {
        for (regex, string) in &self.linkifiers {
            chapter.content = regex
                .replace_all(&chapter.content, |c: &Captures<'_>| -> String {
                    // we add `[]` around it
                    assert!(c[0].starts_with("[") && c[0].ends_with("]"));

                    let mut href = String::new();
                    href.push_str(&c[0]);
                    href.push('(');
                    c.expand(string, &mut href);
                    href.push(')');
                    href
                })
                .to_string();
        }

        Ok(())
    }

    fn link_teams(&self, chapter: &mut Chapter) -> anyhow::Result<()> {
        chapter.content.push_str("\n\n");
        for team in team::get_team_names()? {
            chapter
                .content
                .push_str(&format!("{team}: {}\n", team.url()));
        }
        Ok(())
    }

    /// Replace placeholders like TASK_OWNERS and TEAMS_WITH_ASKS.
    /// All goal documents should have this in their metadata table;
    /// that is enforced during goal parsing.
    fn replace_metadata_placeholders(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        self.replace_metadata_placeholder(chapter, &re::TASK_OWNERS, |goal| {
            goal.task_owners.iter().cloned().collect()
        })?;

        self.replace_metadata_placeholder(chapter, &re::TEAMS_WITH_ASKS, |goal| {
            goal.teams_with_asks()
                .iter()
                .map(|team_name| team_name.name())
                .collect()
        })?;

        Ok(())
    }

    /// Replace one of the placeholders that occur in the goal document metadata,
    /// like [`re::TASK_OWNERS`][].
    fn replace_metadata_placeholder(
        &mut self,
        chapter: &mut Chapter,
        regex: &Regex,
        op: impl Fn(&GoalDocument) -> Vec<String>,
    ) -> anyhow::Result<()> {
        let Some(m) = regex.find(&chapter.content) else {
            return Ok(());
        };
        let range = m.range();

        let Some(chapter_path) = chapter.path.as_ref() else {
            anyhow::bail!(
                "goal chapter `{}` matches placeholder regex but has no path",
                chapter.name
            );
        };

        // Hack: leave this stuff alone in the template
        if chapter_path.file_name().unwrap() == "TEMPLATE.md" {
            return Ok(());
        }

        let goals = self.goal_documents(&chapter_path)?;
        let chapter_in_context = self.ctx.config.book.src.join(chapter_path);
        let Some(goal) = goals.iter().find(|gd| gd.path == chapter_in_context) else {
            anyhow::bail!(
                "goal chapter `{}` has no goal document at path {:?}",
                chapter.name,
                chapter_path,
            );
        };

        let replacement = op(goal).join(", ");
        chapter.content.replace_range(range, &replacement);

        Ok(())
    }
}

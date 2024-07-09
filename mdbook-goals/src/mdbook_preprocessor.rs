use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Context;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::{Captures, Regex};
use walkdir::WalkDir;

use crate::goal::{self, format_team_asks, Status};
use crate::util::GithubUserInfo;

const LINKS: &str = "links";
const LINKIFIERS: &str = "linkifiers";
const USERS: &str = "users";

pub struct GoalPreprocessor;

impl Preprocessor for GoalPreprocessor {
    fn name(&self) -> &str {
        "goals"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        let mut this = GoalPreprocessorWithContext::new(ctx)?;
        for section in &mut book.sections {
            this.process_book_item(section)?;
        }
        Ok(book)
    }
}

pub struct GoalPreprocessorWithContext<'c> {
    team_asks: Regex,
    goal_list: Regex,
    username: Regex,
    ctx: &'c PreprocessorContext,
    links: Vec<(String, String)>,
    linkifiers: Vec<(Regex, String)>,
    display_names: BTreeMap<String, Rc<String>>,
}

impl<'c> GoalPreprocessorWithContext<'c> {
    pub fn new(ctx: &'c PreprocessorContext) -> anyhow::Result<Self> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        let mut links: Vec<(String, String)> = Default::default();
        let mut linkifiers = Default::default();
        let mut display_names = Default::default();
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
        }

        Ok(GoalPreprocessorWithContext {
            ctx,
            team_asks: Regex::new(r"<!-- TEAM ASKS -->")?,
            goal_list: Regex::new(r"<!-- GOALS `(.*)` -->")?,
            username: Regex::new(r"@([-a-zA-Z0-9])+")?,
            links,
            linkifiers,
            display_names,
        })
    }

    fn process_book_item(&mut self, book_item: &mut BookItem) -> anyhow::Result<()> {
        match book_item {
            BookItem::Chapter(chapter) => {
                self.replace_team_asks(chapter)?;
                self.replace_goal_lists(chapter)?;
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

    fn replace_goal_lists(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let Some(m) = self.goal_list.captures(&chapter.content) else {
            return Ok(());
        };
        let range = m.get(0).unwrap().range();
        let status = Status::try_from(&m[1])?;

        let Some(path) = &chapter.path else {
            anyhow::bail!("found `<!-- GOALS -->` but chapter has no path")
        };

        // Extract out the list of goals with the given status.
        let mut goals = vec![];
        for (input, link_path) in self.markdown_files(path)? {
            let opt_metadata = goal::metadata_in_input(&input)
                .with_context(|| format!("extracting metadata from `{}`", input.display()))?;

            if let Some(metadata) = opt_metadata {
                if metadata.status == status {
                    goals.push((metadata, input, link_path));
                }
            }
        }

        //
        let output = goal::format_goal_table(&goals)?;
        chapter.content.replace_range(range, &output);

        // Populate with children if this is not README
        if path.file_stem() != Some("README".as_ref()) {
            let mut parent_names = chapter.parent_names.clone();
            parent_names.push(chapter.name.clone());
            for ((metadata, input, _link_path), index) in goals.iter().zip(0..) {
                let path = input.strip_prefix(&self.ctx.config.book.src).unwrap();
                let content = std::fs::read_to_string(input)
                    .with_context(|| format!("reading `{}`", input.display()))?;
                let mut new_chapter =
                    Chapter::new(&metadata.title, content, path, parent_names.clone());

                if let Some(mut number) = chapter.number.clone() {
                    number.0.push(index + 1);
                    new_chapter.number = Some(number);
                }

                chapter.sub_items.push(BookItem::Chapter(new_chapter));
            }
        }

        self.replace_goal_lists(chapter)
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

        let mut asks_of_any_team = vec![];
        let markdown_files = self.markdown_files(path)?;
        for (input, link_path) in &markdown_files {
            asks_of_any_team.extend(
                goal::team_asks_in_input(input, link_path)
                    .with_context(|| format!("extracting asks from `{}`", input.display()))?,
            );
        }

        let format_team_asks = format_team_asks(&asks_of_any_team)?;

        chapter.content.replace_range(range, &format_team_asks);

        Ok(())
    }

    fn markdown_files(&mut self, chapter_path: &Path) -> anyhow::Result<Vec<(PathBuf, PathBuf)>> {
        let chapter_path = self.ctx.config.book.src.join(chapter_path);
        let parent_path = chapter_path.parent().unwrap();

        let mut files = vec![];
        for entry in WalkDir::new(parent_path) {
            let entry = entry?;

            if entry.file_type().is_file() && entry.path().extension() == Some("md".as_ref()) {
                files.push((
                    entry.path().to_path_buf(),
                    entry
                        .path()
                        .strip_prefix(parent_path)
                        .unwrap()
                        .to_path_buf(),
                ));
            }
        }
        Ok(files)
    }

    fn link_users(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        let usernames: BTreeSet<String> = self
            .username
            .find_iter(&chapter.content)
            .map(|m| m.as_str().to_string())
            .collect();

        for username in &usernames {
            chapter.content = chapter
                .content
                .replace(username, &format!("[{}][]", self.display_name(username)));
        }

        chapter.content.push_str("\n\n");
        for username in &usernames {
            chapter.content.push_str(&format!(
                "[{}]: https://github.com/{}\n",
                self.display_name(username),
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

    fn display_name<'a>(&mut self, username: &str) -> Rc<String> {
        match self.display_names.get(username) {
            Some(n) => n.clone(),
            None => {
                let display_name = Rc::new(
                    match GithubUserInfo::load(username)
                        .with_context(|| format!("loading user info for {}", username))
                    {
                        Ok(GithubUserInfo { name: Some(n), .. }) => n,
                        Ok(GithubUserInfo { name: None, .. }) => username.to_string(),
                        Err(e) => {
                            eprintln!("{:?}", e);
                            username.to_string()
                        }
                    },
                );
                self.display_names
                    .insert(username.to_string(), display_name.clone());
                display_name
            }
        }
    }

    fn linkify(&self, chapter: &mut Chapter) -> anyhow::Result<()> {
        for (regex, string) in &self.linkifiers {
            chapter.content = regex
                .replace_all(&chapter.content, |c: &Captures<'_>| -> String {
                    // we add `[]` around it
                    assert!(c[0].starts_with("[") && c[0].ends_with("]"));

                    eprintln!("c[0] = {}", &c[0]);

                    let mut href = String::new();
                    href.push_str(&c[0]);
                    href.push('(');
                    c.expand(string, &mut href);
                    href.push(')');
                    dbg!(href)
                })
                .to_string();
        }

        Ok(())
    }
}

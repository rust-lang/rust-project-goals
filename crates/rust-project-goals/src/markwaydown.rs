//! Arguably the worst markdown parser ever. Extracts exactly the things we care about.

use std::{fmt::Display, path::Path};

use crate::util;

/// A "section" is a piece of markdown that begins with `##` and which extends until the next section.
/// Note that we don't track the hierarchical structure of sections in particular.
#[derive(Debug)]
pub struct Section {
    /// Line numberin the document
    pub line_num: usize,

    /// Number of hashes
    pub level: usize,

    /// Title of the section -- what came after the `#` in the markdown.
    pub title: String,

    /// Markdown text until start of next section, excluding tables
    pub text: String,

    /// Tables are parsed and stored here
    pub tables: Vec<Table>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    pub line_num: usize,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn parse(path: impl AsRef<Path>) -> anyhow::Result<Vec<Section>> {
    let path = path.as_ref();
    let text = std::fs::read_to_string(path)?;
    parse_text(path, &text)
}

pub fn parse_text(path: impl AsRef<Path>, text: &str) -> anyhow::Result<Vec<Section>> {
    let path: &Path = path.as_ref();
    let mut result = vec![];
    let mut open_section = None;
    let mut open_table = None;

    for (line, line_num) in text.lines().zip(1..) {
        let categorized = categorize_line(line);
        // eprintln!("line = {:?}", line);
        // eprintln!("categorized = {:?}", categorized);
        match categorized {
            CategorizeLine::Title(level, title) => {
                close_section(&mut result, &mut open_section, &mut open_table);
                open_section = Some(Section {
                    line_num,
                    level,
                    title,
                    text: String::new(),
                    tables: vec![],
                });
            }
            CategorizeLine::TableRow(mut row) => {
                if open_section.is_none() {
                    // create an "anonymous" section to house the table
                    open_section = Some(Section {
                        line_num,
                        level: 0,
                        title: String::new(),
                        text: String::new(),
                        tables: vec![],
                    });
                }

                if let Some(table) = &mut open_table {
                    if row.len() > table.header.len() {
                        return Err(anyhow::anyhow!(
                            "{}:{}: too many columns in table, expected no more than {}",
                            path.display(),
                            line_num,
                            table.header.len()
                        ));
                    }

                    while row.len() < table.header.len() {
                        row.push(String::new());
                    }

                    table.rows.push(row);
                } else {
                    open_table = Some(Table {
                        line_num,
                        header: row,
                        rows: vec![],
                    });
                }
            }
            CategorizeLine::TableDashRow(len) => {
                if let Some(table) = &open_table {
                    if table.header.len() != len {
                        return Err(anyhow::anyhow!(
                            "{}:{}: too many columns in table, expected no more than {}",
                            path.display(),
                            line_num,
                            table.header.len()
                        ));
                    }

                    if !table.rows.is_empty() {
                        return Err(anyhow::anyhow!(
                            "{}:{}: did not expect table header here, already saw table rows",
                            path.display(),
                            line_num,
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "{}:{}: did not expect table header here",
                        path.display(),
                        line_num,
                    ));
                }
            }
            CategorizeLine::Other => {
                close_table(&mut open_section, &mut open_table);
                if let Some(section) = open_section.as_mut() {
                    section.text.push_str(line);
                    section.text.push('\n');
                }
            }
        }
    }

    close_section(&mut result, &mut open_section, &mut open_table);

    Ok(result)
}

fn close_table(open_section: &mut Option<Section>, open_table: &mut Option<Table>) {
    if let Some(table) = open_table.take() {
        open_section.as_mut().unwrap().tables.push(table);
    }
}

fn close_section(
    result: &mut Vec<Section>,
    open_section: &mut Option<Section>,
    open_table: &mut Option<Table>,
) {
    close_table(open_section, open_table);
    if let Some(section) = open_section.take() {
        result.push(section);
    }
}

#[derive(Debug)]
enum CategorizeLine {
    Title(usize, String),
    TableRow(Vec<String>),
    TableDashRow(usize),
    Other,
}

fn categorize_line(line: &str) -> CategorizeLine {
    if line.starts_with('#') {
        let level = line.chars().take_while(|&ch| ch == '#').count();
        CategorizeLine::Title(level, line.trim_start_matches('#').trim().to_string())
    } else if line.starts_with('|') && line.ends_with('|') {
        let line = &line[1..line.len() - 1];
        let columns = line.split('|').map(|s| s.trim());
        if columns.clone().all(|s| s.chars().all(|c| c == '-')) {
            CategorizeLine::TableDashRow(columns.count())
        } else {
            CategorizeLine::TableRow(columns.map(|s| s.to_string()).collect())
        }
    } else {
        CategorizeLine::Other
    }
}

impl Table {
    /// For a "key-value" table (like metadata), find an existing row
    /// where the first column (the "key") is `row_key` and modify its second column (the "value")
    /// to be `row_value`. If no row exists with key `row_key`, then add a new row.
    pub fn add_key_value_row(&mut self, row_key: &str, row_value: &impl Display) {
        assert_eq!(self.header.len(), 2);

        match self.rows.iter_mut().find(|row| row[0] == row_key) {
            Some(row) => {
                row[1] = row_value.to_string();
            }

            None => {
                self.rows
                    .push(vec![row_key.to_string(), row_value.to_string()]);
            }
        }
    }

    /// Modify `path` to replace the lines containing this table with `new_table`.
    pub fn overwrite_in_path(&self, path: &Path, new_table: &Table) -> anyhow::Result<()> {
        let full_text = std::fs::read_to_string(path)?;

        let mut new_lines = vec![];
        new_lines.extend(
            full_text
                .lines()
                .take(self.line_num - 1)
                .map(|s| s.to_string()),
        );

        let table_text = {
            let mut new_rows = vec![new_table.header.clone()];
            new_rows.extend(new_table.rows.iter().cloned());
            util::format_table(&new_rows)
        };
        new_lines.push(table_text);

        new_lines.extend(
            full_text
                .lines()
                .skip(self.line_num - 1)
                .skip(2 + self.rows.len())
                .map(|s| s.to_string()),
        );

        let new_text = new_lines.join("\n");
        std::fs::write(path, new_text)?;

        Ok(())
    }
}

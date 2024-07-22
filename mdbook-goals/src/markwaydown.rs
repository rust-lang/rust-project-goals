//! Arguably the worst markdown parser ever. Extracts exactly the things we care about.

use std::path::Path;

#[derive(Debug)]
pub struct Section {
    pub line_num: usize,
    pub title: String,
    pub text: String,
    pub tables: Vec<Table>,
}

#[derive(Debug)]
pub struct Table {
    pub line_num: usize,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn parse(path: &Path) -> anyhow::Result<Vec<Section>> {
    let text = std::fs::read_to_string(path)?;
    let mut result = vec![];
    let mut open_section = None;
    let mut open_table = None;

    for (line, line_num) in text.lines().zip(1..) {
        let categorized = categorize_line(line);
        // eprintln!("line = {:?}", line);
        // eprintln!("categorized = {:?}", categorized);
        match categorized {
            CategorizeLine::Title(title) => {
                close_section(&mut result, &mut open_section, &mut open_table);
                open_section = Some(Section {
                    line_num,
                    title,
                    text: String::new(),
                    tables: vec![],
                });
            }
            CategorizeLine::TableRow(mut row) => {
                if open_section.is_none() {
                    anyhow::bail!(
                        "{}:{}: markdowwn table outside of any section",
                        path.display(),
                        line_num
                    );
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
    Title(String),
    TableRow(Vec<String>),
    TableDashRow(usize),
    Other,
}

fn categorize_line(line: &str) -> CategorizeLine {
    if line.starts_with('#') {
        CategorizeLine::Title(line.trim_start_matches('#').trim().to_string())
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

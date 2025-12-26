use std::{
    collections::BTreeSet,
    fmt::{Display, Write},
    path::{Path, PathBuf},
};

use spanned::{Result, Spanned};
use walkdir::WalkDir;

pub const ARROW: &str = "↳";

pub const MILESTONE_REGEX: &'static str = r"^\d{4}([hH][12])?$";

/// Formats a table as markdown. The input should be a series of rows
/// where each row has the same number of columns.
/// The first row is the headers.
pub fn format_table(rows: &[Vec<Spanned<String>>]) -> String {
    let mut output = String::new();

    let Some((header_row, data_rows)) = rows.split_first() else {
        return String::new();
    };

    let columns = header_row.len();
    let mut widths = vec![0; columns];

    for columns in data_rows {
        for (text, col) in columns.iter().zip(0..) {
            widths[col] = widths[col].max(text.len());
        }
    }

    for (columns, row) in rows.iter().zip(0..) {
        for (text, col) in columns.iter().zip(0..) {
            output.push('|');

            write!(
                output,
                " {text:<width$} ",
                text = **text,
                width = widths[col]
            )
            .unwrap();
        }

        output.push('|');
        output.push('\n');

        // Print the `---` row after the headers
        if row == 0 {
            for width in widths.iter() {
                output.push('|');
                write!(output, " {text:<width$} ", text = ":--", width = width).unwrap();
            }
            output.push('|');
            output.push('\n');
        }
    }

    output
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GithubUserInfo {
    pub name: Option<String>,
}

impl GithubUserInfo {
    pub fn load(login: &str) -> Result<Self> {
        Self::github_request(login)
    }

    fn github_request(login: &str) -> Result<Self> {
        in_thread(|| -> Result<_> {
            // FIXME: cache this in the target directory or something
            use reqwest::header::USER_AGENT;
            let url = format!("https://api.github.com/users/{}", &login[1..]);
            let response: GithubUserInfo = reqwest::blocking::Client::new()
                .get(&url)
                .header(USER_AGENT, "mdbook-goals/1.0")
                .send()?
                .json()?;
            Ok(response)
        })
    }
}

pub fn commas(iter: impl IntoIterator<Item: Display>) -> String {
    let mut output = String::new();
    for (elem, i) in iter.into_iter().zip(0..) {
        if i > 0 {
            write!(output, ", ").unwrap();
        }
        write!(output, "{}", elem).unwrap();
    }
    output
}

/// Returns all markdown files in `directory_path` as `(absolute, relative)` pairs,
/// where `relative` is relative to `directory_path`.
pub fn markdown_files(directory_path: &Path) -> Result<Vec<(PathBuf, PathBuf)>> {
    if !directory_path.is_dir() {
        spanned::bail_here!("`{}` is not a directory", directory_path.display());
    }

    let mut files = vec![];
    for entry in WalkDir::new(directory_path) {
        let entry = entry?;

        if entry.file_type().is_file() && entry.path().extension() == Some("md".as_ref()) {
            files.push((
                entry.path().to_path_buf(),
                entry
                    .path()
                    .strip_prefix(directory_path)
                    .unwrap()
                    .to_path_buf(),
            ));
        }
    }
    Ok(files)
}

/// Returns a comma-separated list of the strings in `s` (no spaces).
pub fn comma(s: &BTreeSet<String>) -> String {
    s.iter().map(|s| &s[..]).collect::<Vec<_>>().join(",")
}

/// Runs `op` in another thread. Useful for making blocking calls to `request`
/// without making tokio upset.
pub fn in_thread<R>(op: impl FnOnce() -> R + Send) -> R
where
    R: Send,
{
    std::thread::scope(|scope| scope.spawn(|| op()).join().unwrap())
}

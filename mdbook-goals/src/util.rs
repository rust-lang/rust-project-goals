use std::fmt::Write;

pub const ARROW: &str = "↳";

/// Formats a table as markdown. The input should be a series of rows
/// where each row has the same number of columns.
/// The first row is the headers.
pub fn format_table(rows: &[Vec<String>]) -> String {
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

            write!(output, " {text:<width$} ", text = text, width = widths[col]).unwrap();
        }

        output.push('|');
        output.push('\n');

        // Print the `---` row after the headers
        if row == 0 {
            for width in widths.iter() {
                output.push('|');
                write!(output, " {text:<width$} ", text = "---", width = width).unwrap();
            }
            output.push('|');
            output.push('\n');
        }
    }

    output
}

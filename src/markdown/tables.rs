use std::io::{Error, Write};
use crate::markdown::{Escaping, MarkdownWritable};

/// A simple markdown table that preserves column order
pub struct MarkdownTable<'a> {
    headers: Vec<String>,
    rows: Vec<Vec<Box<dyn 'a + MarkdownWritable>>>,
}

impl<'a> MarkdownTable<'a> {
    /// Create a new table with specified headers
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }

    pub fn add_string_row(&mut self, rows: Vec<String>) {
        if rows.len() != self.headers.len() {
            panic!("Row length ({}) doesn't match header count ({})", rows.len(), self.headers.len());
        }

        // Convert each String into a Box<dyn MarkdownWritable>
        let converted_row: Vec<Box<dyn 'a + MarkdownWritable>> = rows
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn 'a + MarkdownWritable>)
            .collect();

        self.rows.push(converted_row);
    }
}

impl<'a> MarkdownWritable for MarkdownTable<'a> {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        escape: Escaping,
        line_prefix: Option<&[u8]>,
    ) -> Result<(), Error> {
        // Write header row
        writer.write_all(b"|")?;
        for header in &self.headers {
            writer.write_all(b" ")?;
            header.as_str().write_to(writer, true, escape, line_prefix)?;
            writer.write_all(b" |")?;
        }
        writer.write_all(b"\n")?;

        // Write separator row
        writer.write_all(b"|")?;
        for _ in &self.headers {
            writer.write_all(b"------|")?;
        }
        writer.write_all(b"\n")?;

        // Write data rows
        for row in &self.rows {
            writer.write_all(b"|")?;
            for cell in row {
                writer.write_all(b" ")?;
                cell.write_to(writer, true, escape, line_prefix)?;
                writer.write_all(b" |")?;
            }
            writer.write_all(b"\n")?;
        }

        if !inner {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn count_max_streak(&self, char: u8, _carry: usize) -> (usize, usize) {
        let mut max_count = 0;

        // Check headers
        for header in &self.headers {
            let (count, _) = header.as_str().count_max_streak(char, 0);
            if count > max_count {
                max_count = count;
            }
        }

        // Check rows
        for row in &self.rows {
            for cell in row {
                let (count, _) = cell.count_max_streak(char, 0);
                if count > max_count {
                    max_count = count;
                }
            }
        }
        (max_count, 0)
    }
}

impl MarkdownWritable for String {
    fn write_to(
        &self, writer:
        &mut dyn Write,
        inner: bool,
        escape: Escaping,
        line_prefix: Option<&[u8]>
    ) -> Result<(), Error> {
        self.as_str().write_to(writer, inner, escape, line_prefix)
    }

    fn count_max_streak(
        &self, char: u8,
        carry: usize
    ) -> (usize, usize) {
       self.as_str().count_max_streak(char, carry)
    }
}
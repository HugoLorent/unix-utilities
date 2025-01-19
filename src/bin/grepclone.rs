use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

/// Search for PATTERN in each FILE. Read standard input when no FILE is given
#[derive(Parser)]
struct Cli {
    /// The pattern to search for
    pattern: String,
    /// The paths of the files to search in
    files: Vec<PathBuf>,
    /// Ignore case distinctions in patterns
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut result = String::new();

    if !args.files.is_empty() {
        for file_path in &args.files {
            search_file(file_path, &args.pattern, args.ignore_case, &mut result)?;
        }
    } else {
        let reader = BufReader::new(std::io::stdin().lock());
        if args.ignore_case {
            search_case_insensitive(reader, &args.pattern, &mut result, None)?;
        } else {
            search_case_sensitive(reader, &args.pattern, &mut result, None)?;
        }
    }

    print!("{}", result);

    Ok(())
}

fn search_file(
    file_path: &PathBuf,
    pattern: &str,
    ignore_case: bool,
    result: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)
        .with_context(|| format!("could not read file `{}`", file_path.display()))?;

    let reader = BufReader::new(file);
    let file_name = file_path.file_name().unwrap().to_string_lossy();
    if ignore_case {
        search_case_insensitive(reader, pattern, result, Some(&file_name))?;
    } else {
        search_case_sensitive(reader, pattern, result, Some(&file_name))?;
    }

    Ok(())
}

fn search_case_sensitive<R: BufRead>(
    reader: R,
    pattern: &str,
    result: &mut String,
    file_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            let colored_line = if let Some(file_name) = file_name {
                format!(
                    "{}: {}",
                    file_name.bright_purple(),
                    line.replace(pattern, &format!("{}", pattern.red().bold()))
                )
            } else {
                line.replace(pattern, &format!("{}", pattern.red().bold()))
            };
            result.push_str(&colored_line);
            result.push('\n');
        }
    }
    Ok(())
}

fn search_case_insensitive<R: BufRead>(
    reader: R,
    pattern: &str,
    result: &mut String,
    file_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in reader.lines() {
        let line = line?;
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            // Extract original word in line to colorize it when case insensitive is used
            let pattern_found_index = line.to_lowercase().find(&pattern.to_lowercase()).unwrap();
            let original_pattern =
                &line[pattern_found_index..(pattern_found_index + pattern.len())];

            let colored_line = if let Some(file_name) = file_name {
                format!(
                    "{}: {}",
                    file_name.bright_purple(),
                    line.replace(
                        original_pattern,
                        &format!("{}", original_pattern.red().bold())
                    )
                )
            } else {
                line.replace(
                    original_pattern,
                    &format!("{}", original_pattern.red().bold()),
                )
            };
            result.push_str(&colored_line);
            result.push('\n');
        }
    }
    Ok(())
}

use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufRead, BufReader};

/// Concatenate FILE[s] to standard output.
#[derive(Parser)]
struct Cli {
    /// The paths of the files to concatenate
    files: Vec<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut result = String::new();

    for file_path in &args.files {
        let file = std::fs::File::open(file_path)
            .with_context(|| format!("could not read file `{}`", file_path.display()))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            result.push_str(&line);
            result.push('\n');
        }
    }

    print!("{}", result);
    Ok(())
}

use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufRead, BufReader};

/// Concatenate FILE(s) to standard output. Read standard input when no FILE is given
#[derive(Parser)]
struct Cli {
    /// The paths of the files to concatenate
    #[arg(value_name = "FILE")]
    files: Vec<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut result = String::new();

    if !args.files.is_empty() {
        for file_path in &args.files {
            let file = std::fs::File::open(file_path)
                .with_context(|| format!("could not read file `{}`", file_path.display()))?;
            let reader = BufReader::new(file);
            read_input(reader, &mut result)?;
        }
    } else {
        let reader = BufReader::new(std::io::stdin().lock());
        read_input(reader, &mut result)?;
    }

    print!("{result}");
    Ok(())
}

fn read_input<R: BufRead>(reader: R, result: &mut String) -> Result<(), std::io::Error> {
    for line in reader.lines() {
        let line = line?;
        result.push_str(&line);
        result.push('\n');
    }
    Ok(())
}

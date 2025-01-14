use clap::Parser;
use colored::Colorize;

/// List information about the current directory.
#[derive(Parser)]
struct Cli {
    /// Use a long listing format
    #[arg(short)]
    long: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Cli::parse();
    let dir_entry = std::fs::read_dir(".")?;

    for entry in dir_entry {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            println!("{}  ", entry.file_name().to_string_lossy().blue().bold());
        } else if entry.file_type()?.is_symlink() {
            println!("{}  ", entry.file_name().to_string_lossy().red().bold());
        } else {
            println!("{}  ", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}

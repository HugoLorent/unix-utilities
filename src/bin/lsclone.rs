use std::os::unix::fs::{MetadataExt, PermissionsExt};

use clap::Parser;
use colored::Colorize;
use users::{get_group_by_gid, get_user_by_uid};

/// List information about the current directory.
#[derive(Parser)]
struct Cli {
    /// Use a long listing format
    #[arg(short)]
    long: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let dir_entry = std::fs::read_dir(".")?;

    for entry in dir_entry {
        let entry = entry?;
        print_entry(args.long, entry)?;
    }

    Ok(())
}

fn print_entry(long: bool, entry: std::fs::DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = entry.file_name();

    let file_name = match entry.file_type()? {
        ft if ft.is_dir() => file_name.to_string_lossy().blue().bold(),
        ft if ft.is_symlink() => file_name.to_string_lossy().red().bold(),
        _ => file_name.to_string_lossy().bright_white(),
    };

    if long {
        let metadata = entry.metadata()?;
        let perms = metadata.permissions().mode();
        let perms_str = permissions_to_string(perms);

        let uid = metadata.uid();
        let user = get_user_by_uid(uid).unwrap();
        let user_name = user.name().to_string_lossy();

        let gid = metadata.gid();
        let group = get_group_by_gid(gid).unwrap();
        let group_name = group.name().to_string_lossy();
        let size = metadata.size();
        println!(
            "{} {} {} {} {}",
            perms_str, user_name, group_name, size, file_name
        );
    } else {
        println!("{}  ", file_name);
    }
    Ok(())
}

fn permissions_to_string(mode: u32) -> String {
    let mut perms = String::new();

    // Type de fichier
    if mode & 0o40000 == 0o40000 {
        perms.push('d'); // dossier
    } else if mode & 0o120000 == 0o120000 {
        perms.push('l'); // lien symbolique
    } else if mode & 0o140000 == 0o140000 {
        perms.push('b'); // bloc spécial
    } else if mode & 0o150000 == 0o150000 {
        perms.push('c'); // caractère spécial
    } else {
        perms.push('-'); // fichier régulier
    }

    // Permissions utilisateur
    if mode & 0o400 == 0o400 {
        perms.push('r');
    } else {
        perms.push('-');
    }
    if mode & 0o200 == 0o200 {
        perms.push('w');
    } else {
        perms.push('-');
    }
    if mode & 0o100 == 0o100 {
        perms.push('x');
    } else {
        perms.push('-');
    }

    // Permissions groupe
    if mode & 0o040 == 0o040 {
        perms.push('r');
    } else {
        perms.push('-');
    }
    if mode & 0o020 == 0o020 {
        perms.push('w');
    } else {
        perms.push('-');
    }
    if mode & 0o010 == 0o010 {
        perms.push('x');
    } else {
        perms.push('-');
    }

    // Permissions autres
    if mode & 0o004 == 0o004 {
        perms.push('r');
    } else {
        perms.push('-');
    }
    if mode & 0o002 == 0o002 {
        perms.push('w');
    } else {
        perms.push('-');
    }
    if mode & 0o001 == 0o001 {
        perms.push('x');
    } else {
        perms.push('-');
    }

    perms
}

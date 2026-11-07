use std::{os::unix::fs::PermissionsExt};

fn list_directory(path: &str) {
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
            return;
        },
    };

    println!();
    println!("PERMS:\t\t  MODE:\t\tNAME:");
    println!("-------------------------------------");

    for entry in entries.into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let metadata = match std::fs::symlink_metadata(entry.path()) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let permissions = metadata.permissions();

        let mode = permissions.mode();
        let mut chars = String::new();

        let (is_file, is_dir, is_lnk) = (
            metadata.is_file(),
            metadata.is_dir(),
            metadata.is_symlink(),
        );

        if is_lnk {
            chars.push('l');
        } else if is_dir {
            chars.push('d');
        } else if is_file {
            chars.push('-');
        } else {
            chars.push('?');
        }

        for &shift in &[6, 3, 0] {
            chars.push(if (mode & (4 << shift)) != 0 {'r'} else {'-'});
            chars.push(if (mode & (2 << shift)) != 0 {'w'} else {'-'});
            chars.push(if (mode & (1 << shift)) != 0 {'x'} else {'-'});
        }

        chars.push('\t');
        let file_name = entry.file_name().to_string_lossy().into_owned();

        let output_string = format!("{}  {:o}\t  \t{}", chars, (permissions.mode() & 0o777), file_name);
        println!("{}", output_string);
    }

    println!();
}   

fn main() {
    let args: Vec<String> = std::env::args().collect();
        
    if args.len() == 1 {
        list_directory(".");
        return;
    } else if args.len() == 2 {
        list_directory(&args[1]);
        return;
    } else {
        println!("\nUsage: ls_rs => [path]\n");
    }
}

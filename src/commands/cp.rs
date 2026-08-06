use std::fs;
use std::path::{Path, PathBuf};

pub fn run(source: &str, destination: &str) {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if !source_path.exists() {
        eprintln!("cp: {}: No such file or directory", source);
        return;
    }

    // file -> directory
    let final_destination = if destination_path.is_dir() {
        let filename = match source_path.file_name() {
            Some(name) => name,
            None => {
                eprintln!("cp: invalid source");
                return;
            }
        };

        destination_path.join(filename)
    } else {
        PathBuf::from(destination)
    };


    // ما ندعموش directory
    if source_path.is_dir() {
        eprintln!("cp: {} is a directory", source);
        return;
    }


    match fs::copy(source_path, final_destination) {
        Ok(_) => {}

        Err(err) => {
            eprintln!("cp: {}", err);
        }
    }
}
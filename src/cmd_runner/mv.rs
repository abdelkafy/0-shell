use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("mv: missing operand");
        return;
    }

    let destination = Path::new(&args[args.len() - 1]);
    let sources = &args[..args.len() - 1];

    if sources.len() > 1 && !destination.is_dir() {
        eprintln!(
            "mv: target '{}': No such file or directory",
            destination.display()
        );
        return;
    }

    for source in sources {
        let source_path = Path::new(source);

        let final_destination = if destination.is_dir() {
            match source_path.file_name() {
                Some(filename) => destination.join(filename),
                None => {
                    eprintln!("mv: invalid source '{}'", source);
                    continue;
                }
            }
        } else {
            destination.to_path_buf()
        };

        match fs::rename(source_path, &final_destination) {
            Ok(_) => {}

            Err(err) => {
                eprintln!(
                    "mv: cannot move '{}' to '{}': {}",
                    source,
                    final_destination.display(),
                    err
                );
            }
        }
    }
}
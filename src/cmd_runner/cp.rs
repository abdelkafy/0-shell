use crate::errors::format_error;

use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("cp: missing file operand");
        return;
    }

    if args.len() == 1 {
        eprintln!(
            "cp: missing destination file operand after '{}'",
            args[0]
        );
        return;
    }

    let destination = PathBuf::from(&args[args.len() - 1]);
    let sources = &args[..args.len() - 1];

    if sources.len() > 1 && !destination.is_dir() {
        eprintln!(
            "cp: target '{}': Not a directory",
            destination.display()
        );
        return;
    }

    for source in sources {
        let source_path = Path::new(source);

        if !source_path.exists() {
            eprintln!(
                "cp: cannot stat '{}': {}",
                source,
                format_error(&std::io::Error::from(
                    std::io::ErrorKind::NotFound
                ))
            );
            continue;
        }

        if source_path.is_dir() {
            eprintln!(
                "cp: -r not specified; omitting directory '{}'",
                source
            );
            continue;
        }

        let target = if destination.is_dir() {
            match source_path.file_name() {
                Some(name) => destination.join(name),

                None => {
                    eprintln!("cp: invalid source '{}'", source);
                    continue;
                }
            }
        } else {
            destination.clone()
        };

        if let (Ok(source_canonical), Ok(target_canonical)) = (
            fs::canonicalize(source_path),
            fs::canonicalize(&target),
        ) {
            if source_canonical == target_canonical {
                eprintln!(
                    "cp: '{}' and '{}' are the same file",
                    source,
                    target.display()
                );
                continue;
            }
        }

        if let Err(err) = fs::copy(source_path, &target) {
            eprintln!(
                "cp: cannot create regular file '{}': {}",
                target.display(),
                format_error(&err)
            );
        }
    }
}

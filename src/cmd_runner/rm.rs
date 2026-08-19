use crate::errors::format_error;
use std::fs;
use std::path::{Component, Path};

pub fn run(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    let mut recursive = false;
    let mut targets = Vec::new();

    for arg in &args {
        match arg.as_str() {
            "-r" => {
                recursive = true;
            }

            _ => {
                targets.push(arg);
            }
        }
    }

    if targets.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    for target in targets {
        let path = Path::new(target);

        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,

            Err(err) => {
                eprintln!(
                    "rm: cannot remove '{}': {}",
                    target,
                    format_error(&err)
                );
                continue;
            }
        };

        if metadata.is_dir() {
            if !recursive {
                eprintln!(
                    "rm: cannot remove '{}': {}",
                    target,
                    "Is a directory"
                );
                continue;
            }

            if is_dot_or_dotdot(path) {
                eprintln!(
                    "rm: refusing to remove '.' or '..' directory: skipping '{}'",
                    target
                );
                continue;
            }

            let affects_current_dir = is_current_dir_inside(path);

            if let Err(err) = fs::remove_dir_all(path) {
                eprintln!(
                    "rm: cannot remove '{}': {}",
                    target,
                    format_error(&err)
                );
            } else if affects_current_dir {
                go_home();
            }
        } else {
            if let Err(err) = fs::remove_file(path) {
                eprintln!(
                    "rm: cannot remove '{}': {}",
                    target,
                    format_error(&err)
                );
            }
        }
    }
}

fn is_dot_or_dotdot(path: &Path) -> bool {
    path.components().last().is_some_and(|component| {
        matches!(
            component,
            Component::CurDir | Component::ParentDir
        )
    })
}

fn is_current_dir_inside(target: &Path) -> bool {
    let current = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => return false,
    };

    let target = match fs::canonicalize(target) {
        Ok(path) => path,
        Err(_) => return false,
    };

    current.starts_with(&target)
}

fn go_home() {
    if let Ok(home) = std::env::var("HOME") {
        let _ = std::env::set_current_dir(home);
    }
}
use std::fs;
use std::io::ErrorKind;

pub fn run(names: Vec<String>) {
    if names.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for name in names {
        match fs::create_dir(&name) {
            Ok(_) => {}

            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!(
                        "mkdir: cannot create directory '{}': File exists",
                        name
                    );
                }

                ErrorKind::NotFound => {
                    eprintln!(
                        "mkdir: cannot create directory '{}': No such file or directory",
                        name
                    );
                }

                ErrorKind::PermissionDenied => {
                    eprintln!(
                        "mkdir: cannot create directory '{}': Permission denied",
                        name
                    );
                }

                _ => {
                    eprintln!(
                        "mkdir: cannot create directory '{}': {}",
                        name,
                        err
                    );
                }
            },
        }
    }
}
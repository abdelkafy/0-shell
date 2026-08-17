use crate::errors::format_error;
use std::fs::File;
use std::io::{self, BufReader, Write};

pub fn run(args: Vec<String>) {
    for path in args {
        match File::open(&path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);

                if let Err(err) = io::copy(&mut reader, &mut io::stdout()) {
                    eprintln!("cat: read error: {}", format_error(&err));
                    return;
                }

                let _ = io::stdout().flush();
            }

            Err(err) => {
                match err.kind() {
                    io::ErrorKind::IsADirectory => {
                        eprintln!("cat: read error: Is a directory");
                    }

                    _ => {
                        eprintln!(
                            "cat: can't open '{}': {}",
                            path,
                            format_error(&err)
                        );
                    }
                }
            }
        }
    }
}

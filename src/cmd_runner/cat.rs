use crate::errors::format_error;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::fs::File;
use std::io::{self, BufReader, Write};

pub fn run(args: Vec<String>) {
    if args.is_empty() {
        run_stdin();
        return;
    }

    for path in args {
        if path == "-" {
            run_stdin();
            continue;
        }

        match File::open(&path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut data = Vec::new();

                if let Err(err) =
                    std::io::Read::read_to_end(&mut reader, &mut data)
                {
                    eprintln!("cat: read error: {}", format_error(&err));
                    return;
                }

                if let Err(err) = io::stdout().write_all(&data) {
                    eprintln!("cat: write error: {}", format_error(&err));
                    return;
                }

                if !data.ends_with(b"\n") {
                    if let Err(err) = io::stdout().write_all(b"%\n") {
                        eprintln!("cat: write error: {}", format_error(&err));
                        return;
                    }
                }

                let _ = io::stdout().flush();
            }

            Err(err) => match err.kind() {
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
            },
        }
    }
}
fn run_stdin() {
    let mut buffer = String::new();
    print!("\r");
    io::stdout().flush().unwrap();

    loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    print!("^C\r\n");
                    io::stdout().flush().unwrap();
                    return;
                }

                if key.code == KeyCode::Char('d') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    print!("\r\n");
                    io::stdout().flush().unwrap();
                    return;
                }

                match key.code {
                    KeyCode::Enter => {
                        print!("\r\n");

                        buffer.clear();

                        io::stdout().flush().unwrap();
                    }

                    KeyCode::Char(c) => {
                        buffer.push(c);

                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }

                    KeyCode::Backspace => {
                        if buffer.pop().is_some() {
                            print!("\x08 \x1b[K");
                            io::stdout().flush().unwrap();
                        }
                    }

                    _ => {}
                }
            }

            Ok(_) => {}

            Err(err) => {
                eprintln!("cat: read error: {}", err);
                return;
            }
        }
    }
}
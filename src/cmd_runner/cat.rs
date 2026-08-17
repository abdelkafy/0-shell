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
        match File::open(&path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
            
                if let Err(err) = io::copy(&mut reader, &mut io::stdout()) {
                    eprintln!("cat: read error: {}", format_error(&err));
                    return;
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

    loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    print!("^C\r\n");
                    io::stdout().flush().unwrap();
                    return;
                }

                if key.code == KeyCode::Char('d')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    print!("\r\n");
                    io::stdout().flush().unwrap();
                    return;
                }

                match key.code {
                    KeyCode::Enter => {
                        print!("\r\n");

                        print!("{}", buffer);

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


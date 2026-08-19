mod cmd_runner;
mod models;
mod cmd_manager;
mod errors;

pub mod parser;

use crate::cmd_manager::executor::command_executor;
use crate::models::ShellPath;
use crate::parser::parser::parser;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        Clear,
        ClearType,
    },
};

use std::io::{self, Write};

fn main() {
    enable_raw_mode().unwrap();

    let mut shell_path = ShellPath::new();
    let mut input = String::new();

    print_prompt();

    loop {
        match event::read() {
            Ok(Event::Key(key)) => {


                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    input.clear();

                    print!("^C\r\n");
                    print_prompt();

                    continue;
                }


                if key.code == KeyCode::Char('d') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    if input.is_empty() {
                        break;
                    }

                    input.clear();

                    print!("\r\x1b[2K");
                    print_prompt();

                    continue;
                }


                match key.code {


                    KeyCode::Enter => {
                        print!("\r\n");
                        io::stdout().flush().unwrap();


                        if needs_more_input(&input) {
                            input.push('\n');

                            print!("> ");
                            io::stdout().flush().unwrap();

                            continue;
                        }

                        let command_input = input.clone();

                        input.clear();

                        if !command_input.trim().is_empty() {

                            disable_raw_mode().unwrap();

                            match parser(&command_input) {
                                Ok((command, args)) => {
                                    command_executor(
                                        command,
                                        args,
                                        &mut shell_path,
                                    );
                                }

                                Err(err) => {
                                    if !err.is_empty() {
                                        print!("{}", err);
                                    }
                                }
                            }

                            io::stdout().flush().unwrap();

                            enable_raw_mode().unwrap();
                        }

                        print_prompt();
                    }


                    KeyCode::Backspace => {
                        if input.pop().is_some() {
                            execute!(
                                io::stdout(),
                                cursor::MoveLeft(1),
                                Clear(ClearType::UntilNewLine)
                            )
                            .unwrap();

                            io::stdout().flush().unwrap();
                        }
                    }


                    KeyCode::Char(c) => {
                        input.push(c);

                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }

                    _ => {}
                }
            }

            Ok(_) => {}

            Err(err) => {
                eprintln!("terminal error: {}", err);
                break;
            }
        }
    }

    disable_raw_mode().unwrap();

    println!();
}

fn print_prompt() {
    let current = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            print!("\r\x1b[2K$ ");
            io::stdout().flush().unwrap();
            return;
        }
    };

    let home = std::env::var("HOME").unwrap_or_default();

    let path = current
        .display()
        .to_string()
        .replace(&home, "~");

    print!(
        "\r\x1b[2K\x1b[34m{}\x1b[0m \x1b[32m$ \x1b[0m",
        path
    );

    io::stdout().flush().unwrap();
}

fn needs_more_input(input: &str) -> bool {
    has_unclosed_quotes(input)
        || ends_with_unescaped_backslash(input)
}

fn has_unclosed_quotes(input: &str) -> bool {
    let mut single_quote = false;
    let mut double_quote = false;
    let mut backtick = false;
    let mut escaped = false;

    for c in input.chars() {
        if escaped {
            escaped = false;
            continue;
        }

        if c == '\\' && !single_quote {
            escaped = true;
            continue;
        }

        match c {
            '\'' if !double_quote && !backtick => {
                single_quote = !single_quote;
            }

            '"' if !single_quote && !backtick => {
                double_quote = !double_quote;
            }

            '`' if !single_quote && !double_quote => {
                backtick = !backtick;
            }

            _ => {}
        }
    }

    single_quote || double_quote || backtick
}
fn ends_with_unescaped_backslash(input: &str) -> bool {
    let line = input.trim_end_matches(['\n', '\r']);

    let mut count = 0;

    for c in line.chars().rev() {
        if c == '\\' {
            count += 1;
        } else {
            break;
        }
    }

    count % 2 == 1
}

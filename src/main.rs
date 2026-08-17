mod cmd_runner;
mod models;
mod cmd_manager;
mod errors;

pub mod parser;

use std::io::{self, Write};

use crate::cmd_manager::executor::command_executor;
use crate::models::ShellPath;
use crate::parser::parser::parser;

fn main() {
    let mut shell_path = ShellPath::new();

    loop {
        print_prompt();
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }

            Ok(_) => {
                match parser(&input) {
                    Ok((command, args)) => {
                        command_executor(command, args, &mut shell_path);
                    }

                    Err(err) => {
                        print!("{}", err);
                    }
                }
            }

            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
}

fn needs_more_input(input: &str) -> bool {
    has_unclosed_quotes(input) || ends_with_unescaped_backslash(input)
}

fn has_unclosed_quotes(input: &str) -> bool {
    let mut single_quote = false;
    let mut double_quote = false;
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

        if c == '\'' && !double_quote {
            single_quote = !single_quote;
        } else if c == '"' && !single_quote {
            double_quote = !double_quote;
        }
    }

    single_quote || double_quote
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

fn print_prompt() {
    let current = std::env::current_dir().unwrap();
    let home = std::env::var("HOME").unwrap_or_default();

    let path = current
        .display()
        .to_string()
        .replace(&home, "~");

    print!("\x1b[34m{}\x1b[0m \x1b[32m$ \x1b[0m", path);

    io::stdout().flush().unwrap();
}
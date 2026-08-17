mod cmd_runner;
mod models;
mod cmd_manager;
pub mod parser;
use std::io::{self, Write};
use crate::cmd_manager::executor::command_executor;
use crate::parser::parser::parser;

fn main() {
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
                while has_unclosed_quotes(&input) {
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut next_line = String::new();

                    match io::stdin().read_line(&mut next_line) {
                        Ok(0) => {
                            println!();
                            break;
                        }

                        Ok(_) => {
                            input.push_str(&next_line);
                        }

                        Err(err) => {
                            eprintln!("{err}");
                            break;
                        }
                    }
                }

                match parser(&input) {
                    Ok((command, args)) => {
                        command_executor(command, args);
                    }

                    Err(err) => {
                        if err != "empty input" {
                            eprintln!("{err}");
                        }
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


use std::env;

fn print_prompt() {
    let current = env::current_dir().unwrap();
    let home = env::var("HOME").unwrap_or_default();

    let path = current
        .display()
        .to_string()
        .replace(&home, "~");

    print!("\x1b[34m{}\x1b[0m \x1b[32m$ \x1b[0m", path);

    io::stdout().flush().unwrap();
}
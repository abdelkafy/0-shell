mod commands;
mod models;
mod execute;
pub mod parser;
use std::io::{self, Write};
use crate::models::Command;
use crate::parser::parser::parser;
use crate::execute::executor::execute;

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
                match parser(&input) {
                    Ok(Command::Exit) => break,
                    Ok(cmd) => execute(cmd),
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
                    }
                };

            }
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
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
mod cmd_runner;
mod models;
mod cmd_manager;
pub mod parser;
use std::io::{self, Write};
use crate::models::Command;
use crate::parser::parser::parser;

fn main() {

    loop {
        print!("$ ");
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
                    Ok(cmd) => (cmd),
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

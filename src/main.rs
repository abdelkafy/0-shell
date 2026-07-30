mod models;
pub mod parser;
use std::io::{self, Write};
use crate::models::{Command, LsFlag};
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
                let command = match parser(&input) {
                    Ok(command) => command,
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

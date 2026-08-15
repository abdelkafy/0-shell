mod cmd_runner;
mod models;
mod cmd_manager;
pub mod parser;
use std::io::{self, Write};
use crate::cmd_manager::executor::command_executor;
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
               match parser(&input)  {
                   Ok((command,args))=>command_executor(command,args),
                   Err(str)=>print!("{}",str),
               }
            }
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
}

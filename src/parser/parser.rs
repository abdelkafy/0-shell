
use std::fmt::Error;

use crate::{cmd_manager::managers::{ls_manager,pwd_manager}, models::{Command}};
pub fn parser(input: &str) ->Result<(Command,Vec<String>),Err> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input")
    }

    let tokens = trimmed.split_whitespace();
    let args:Vec< String>=tokens.map(|str|str.to_string()).collect();
  let name=args[0].as_str();
    let args=args[1..].to_vec();
    match name {
        "echo" => Ok((Command::Echo,args)),
        "cd" => Ok((Command::Cd,args)),
        "ls" => Ok((Command::Ls,args)),
        "pwd" => Ok((Command::Pwd,args)),
        "cat" => Ok((Command::Cat,args)),
        "cp" => Ok((Command::Cp,args)),
        "rm" => Ok((Command::Rm,args)),
        "mv" => Ok((Command::Mv,args)),
        "mkdir" => Ok((Command::Mkdir,args)),
        "exit" => Ok((Command::Exit,args)),
        _ => Err("Command dont exist"),
    }
}


use crate::{cmd_manager::managers::{ls_manager,pwd_manager}, models::{Command}};
pub fn parser(input: &str)  {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return 
    }

    let tokens = trimmed.split_whitespace();
    let args:Vec< String>=tokens.map(|str|str.to_string()).collect();
  let name=args[0].as_str();

    match name {
        "echo" => pwd_manager(),
        "cd" => pwd_manager(),
        "ls" => {
            ls_manager(args[1..].to_vec());
        },
        "pwd" => pwd_manager(),
        "cat" => pwd_manager(),
        "cp" => pwd_manager(),
        "rm" => {
            pwd_manager()
        }
        "mv" => pwd_manager(),
        "mkdir" => pwd_manager(),
        "exit" => pwd_manager(),
        _ => print!("err"),
    }
}

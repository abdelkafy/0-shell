
use crate::{cmd_manager::managers::ls_manager, models::{Command, models::Ls}};
pub fn parser(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input".into());
    }

    let tokens = trimmed.split_whitespace();
    let args:Vec< String>=tokens.map(|str|str.to_string()).collect();
  let name=args[0].as_str();

    match name {
        "echo" => Ok(Command::Echo),
        "cd" => {
            Ok(Command::Cd)
        }
        "ls" => {
            ls_manager(args[1..].to_vec());
            Ok(Command::Echo)
        },
        "pwd" => Ok(Command::Pwd),
        "cat" => Ok(Command::Cat),
        "cp" => Ok(Command::Cp),
        "rm" => {
            Ok(Command::Rm)
        }
        "mv" => Ok(Command::Mv),
        "mkdir" => Ok(Command::Mkdir),
        "exit" => Ok(Command::Exit),
        _ => Err("err".to_string()),
    }
}

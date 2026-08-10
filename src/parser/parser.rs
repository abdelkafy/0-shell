use crate::models::{Command, models::Ls};

pub fn parser(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input".into());
    }

    let tokens = trimmed.split_whitespace();
  let name=tokens.collect::<Vec<_>>()[0];

    match name {
        "echo" => Ok(Command::Echo),
        "cd" => {
            Ok(Command::Cd)
        }
        "ls" => Ok(Command::Ls(Ls{
            classify:true, all:true, long:true,path:".".to_string()
        })),
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

use crate::models::{Command, LsFlag};

pub fn parser(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input".into());
    }

    let tokens = trimmed.split_whitespace();
  let name=tokens.collect::<Vec<_>>()[0];

    match name.to_ascii_lowercase().as_str() {
        "echo" => Ok(Command::echo),
        "cd" => {
            Ok(Command::cd)
        }
        "ls" => Ok(Command::ls((LsFlag::all))),
        "pwd" => Ok(Command::pwd),
        "cat" => Ok(Command::cat),
        "cp" => Ok(Command::cp),
        "rm" => {
            Ok(Command::rm)
        }
        "mv" => Ok(Command::mv),
        "mkdir" => Ok(Command::mkdir),
        "exit" => Ok(Command::exit),
        _ => Err("err".to_string()),
    }
}

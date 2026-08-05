use crate::models::{Command, Lsflag};

pub fn parser(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input".into());
    }

    let mut tokens = trimmed.split_whitespace();
    let name = tokens.next().ok_or("empty input")?;

    match name.to_ascii_lowercase().as_str() {
        "echo" => {
            let args = tokens.map(|s| s.to_string()).collect();
            Ok(Command::Echo(args))
        }
        "cd" => {
            let path = tokens.next().ok_or("cd: missing operand")?;
            Ok(Command::Cd(path.to_string()))
        }
        "ls" => Ok(Command::Ls(Lsflag::All)),
        "pwd" => Ok(Command::Pwd),
        "cat" => {
            let file = tokens.next().ok_or("cat: missing operand")?;
            Ok(Command::Cat(file.to_string()))
        }
        "cp" => {
            let source = tokens.next()
                .ok_or("cp: missing source")?;

            let destination = tokens.next()
                .ok_or("cp: missing destination")?;

            Ok(Command::Cp(
                source.to_string(),
                destination.to_string()
            ))
        }
        "rm" => {
            Ok(Command::Rm)
        }
        "mv" => Ok(Command::Mv),
        "mkdir" => {
            let name = tokens.next().ok_or("mkdir: missing operand")?;
            Ok(Command::Mkdir(name.to_string()))
        }
        "exit" => Ok(Command::Exit),
        _ => Err("err".to_string()),
    }
}


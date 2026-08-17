use crate::models::Command;

pub fn parser(input: &str) -> Result<(Command, Vec<String>), String> {
    let tokens = tokenize(input)?;

    if tokens.is_empty() {
        return Err("empty input".to_string());
    }

    let name = tokens[0].as_str();
    let args = tokens[1..].to_vec();

    match name {
        "echo" => Ok((Command::Echo, args)),
        "cd" => Ok((Command::Cd, args)),
        "ls" => Ok((Command::Ls, args)),
        "pwd" => Ok((Command::Pwd, args)),
        "cat" => Ok((Command::Cat, args)),
        "cp" => Ok((Command::Cp, args)),
        "rm" => Ok((Command::Rm, args)),
        "mv" => Ok((Command::Mv, args)),
        "mkdir" => Ok((Command::Mkdir, args)),
        "exit" => Ok((Command::Exit, args)),
        _ => Err(format!("sh: {name}: not found")),
    }
}

fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    let mut single_quote = false;
    let mut double_quote = false;
    let mut escaped = false;

    for c in input.chars() {
        // Backslash
        if escaped {
            current.push(c);
            escaped = false;
            continue;
        }

        if c == '\\' && !single_quote {
            escaped = true;
            continue;
        }

        // Inside single quote
        if single_quote {
            if c == '\'' {
                single_quote = false;
            } else {
                current.push(c);
            }

            continue;
        }

        // Inside double quote
        if double_quote {
            if c == '"' {
                double_quote = false;
            } else {
                // IMPORTANT:
                // spaces/newlines are added to the SAME token
                current.push(c);
            }

            continue;
        }

        // Outside quotes
        match c {
            '\'' => {
                single_quote = true;
            }

            '"' => {
                double_quote = true;
            }

            ' ' | '\t' | '\n' => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }

            _ => {
                current.push(c);
            }
        }
    }

    if escaped {
        current.push('\\');
    }

    if single_quote {
        return Err("sh: unmatched single quote".to_string());
    }

    if double_quote {
        return Err("sh: unmatched double quote".to_string());
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

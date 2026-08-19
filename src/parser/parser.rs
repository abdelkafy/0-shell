use crate::models::Command;

pub fn parser(input: &str) -> Result<(Command, Vec<String>), String> {
    let tokens = tokenize(input)?;

    if tokens.is_empty() {
        return Err("".to_string());
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
        _ => Err(format!("Command '{name}' not found\n")),
    }
}


fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    let mut single_quote = false;
    let mut double_quote = false;
    let mut backtick = false;
    let mut token_started = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if single_quote {
            if c == '\'' {
                single_quote = false;
            } else {
                current.push(c);
            }

            continue;
        }

        if backtick {
            if c == '`' {
                backtick = false;
            } else {
                current.push(c);
            }

            continue;
        }

        if double_quote {
            match c {
                '"' => {
                    double_quote = false;
                }

                '\\' => {
                    match chars.peek() {
                        Some('$') | Some('`') | Some('"') | Some('\\') => {
                            current.push(chars.next().unwrap_or_default());
                        }

                        Some('\n') => {
                            chars.next();
                        }

                        Some('\r') => {
                            chars.next();

                            if chars.peek() == Some(&'\n') {
                                chars.next();
                            }
                        }

                        Some(_) | None => {
                            current.push('\\');
                        }
                    }
                }

                _ => {
                    current.push(c);
                }
            }

            continue;
        }

        match c {
            '\'' => {
                single_quote = true;
                token_started = true;
            }

            '"' => {
                double_quote = true;
                token_started = true;
            }

            '`' => {
                backtick = true;
                token_started = true;
            }

            '#'
                if !token_started
                    || current
                        .chars()
                        .last()
                        .map(|c| c.is_whitespace())
                        .unwrap_or(false) =>
            {
                while let Some(next) = chars.next() {
                    if next == '\n' {
                        break;
                    }
                }

                if token_started {
                    tokens.push(std::mem::take(&mut current));
                    token_started = false;
                }

                break;
            }

            '\\' => {
                token_started = true;

                match chars.peek() {
                    Some('\n') => {
                        chars.next();
                    }

                    Some('\r') => {
                        chars.next();

                        if chars.peek() == Some(&'\n') {
                            chars.next();
                        }
                    }

                    Some(_) => {
                        current.push(chars.next().unwrap_or_default());
                    }

                    None => {
                        current.push('\\');
                    }
                }
            }

            ' ' | '\t' | '\n' | '\r' => {
                if token_started {
                    tokens.push(std::mem::take(&mut current));
                    token_started = false;
                }
            }

            _ => {
                token_started = true;
                current.push(c);
            }
        }
    }

    if single_quote {
        return Err("sh: unmatched single quote".to_string());
    }

    if double_quote {
        return Err("sh: unmatched double quote".to_string());
    }

    if backtick {
        return Err("sh: unmatched backtick".to_string());
    }

    if token_started {
        tokens.push(current);
    }

    Ok(tokens)
}
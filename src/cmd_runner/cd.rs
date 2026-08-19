use crate::errors::format_error;
use crate::models::ShellPath;
use std::path::PathBuf;
use std::env;

pub fn run(args: Vec<String>, shell_path: &mut ShellPath) {
    let args: &[String] = if args.first().map(String::as_str) == Some("--") {
        &args[1..]
    } else {
        &args[..]
    };

    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    let home = match env::var("HOME") {
        Ok(home) => PathBuf::from(home),

        Err(_) => {
            eprintln!("cd: HOME not set");
            return;
        }
    };

    let target = match args.first().map(String::as_str) {
        None => home.clone(),

        Some("-") => {
            match &shell_path.previous {
                Some(path) => path.clone(),

                None => {
                    eprintln!("cd: OLDPWD not set");
                    return;
                }
            }
        }

        Some("~") => home.clone(),

        Some(path) if path.starts_with("~/") => {
            home.join(&path[2..])
        }
        Some(path) => PathBuf::from(path),
    };

    let old_current = match env::current_dir() {
        Ok(path) => path,

        Err(err) => {
            eprintln!("cd: {}", format_error(&err));
            return;
        }
    };

    match env::set_current_dir(&target) {
        Ok(_) => {}

        Err(err) => {
            eprintln!(
                "cd: {}: {}",
                target.display(),
                format_error(&err)
            );

            return;
        }
    }

    let new_current = match env::current_dir() {
        Ok(path) => path,

        Err(err) => {
            eprintln!("cd: {}", format_error(&err));
            return;
        }
    };

    shell_path.previous = Some(old_current);
    shell_path.current = new_current.clone();

    if args.first().map(String::as_str) == Some("-") {
        println!("{}", new_current.display());
    }
}

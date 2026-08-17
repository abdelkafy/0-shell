use crate::errors::format_error;
use crate::models::ShellPath;
use std::env;
use std::path::PathBuf;

pub fn run(args: Vec<String>, shell_path: &mut ShellPath) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    let target = match args.first().map(String::as_str) {
        // cd
        None => match dirs::home_dir() {
            Some(home) => home,
            None => {
                eprintln!("cd: HOME not set");
                return;
            }
        },

        // cd ~
        Some("~") => match dirs::home_dir() {
            Some(home) => home,
            None => {
                eprintln!("cd: HOME not set");
                return;
            }
        },

        // cd -
        Some("-") => shell_path.previous.clone(),

        // cd <path>
        Some(path) => PathBuf::from(path),
    };

    let old_current = shell_path.current.clone();

    match env::set_current_dir(&target) {
        Ok(_) => {
            shell_path.previous = old_current;

            shell_path.current = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("cd: {}", format_error(&err));
                    return;
                }
            };

            // BusyBox: cd - prints the new directory
            if args.first().map(String::as_str) == Some("-") {
                println!("{}", shell_path.current.display());
            }
        }

        Err(err) => {
            eprintln!(
                "cd: {}: {}",
                target.display(),
                format_error(&err)
            );
        }
    }
}

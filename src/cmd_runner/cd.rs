use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn run(args: Vec<String>) {
    let mut end_of_options = false;
    let mut path: Option<String> = None;

    for arg in args {
        if !end_of_options && arg == "--" {
            end_of_options = true;
            continue;
        }

        if !end_of_options && arg.starts_with('-') {
            eprintln!("cd: invalid option: {}", arg);
            return;
        }

        if path.is_some() {
            eprintln!("cd: too many arguments");
            return;
        }

        path = Some(arg);
    }

    let path = path.unwrap_or_else(|| "~".to_string());
    let target = expand_home(&path);

    if let Err(err) = env::set_current_dir(&target) {
        let message = match err.kind() {
            ErrorKind::NotFound => "no such file or directory",
            ErrorKind::PermissionDenied => "permission denied",
            ErrorKind::NotADirectory => "not a directory",
            _ => "unknown error",
        };

        eprintln!("cd: {}: {}", message, path);
    }
}

fn expand_home(path: &str) -> PathBuf {
    if path == "~" {
        return env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(path));
    }

    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }

    PathBuf::from(path)
}

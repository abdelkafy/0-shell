use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn run(path: &str) {
    let target = expand_home(path);

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
    let path = if path.is_empty() { "~" } else { path };

    if path == "~" {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home);
        }
    }

    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }

    PathBuf::from(path)
}
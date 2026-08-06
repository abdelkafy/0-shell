use std::env;
use std::path::PathBuf;

pub fn run(path: &str) {
    let target = expand_home(path);

    match env::set_current_dir(&target) {
        Ok(_) => {}

        Err(err) => {
            eprintln!("cd: {}: {}", path, err);
        }
    }
}

fn expand_home(path: &str) -> PathBuf {
    if path == "~" {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home);
        }
    }

    if path.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(&path[2..]);
        }
    }

    PathBuf::from(path)
}
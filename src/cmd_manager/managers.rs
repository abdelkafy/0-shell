use crate::{cmd_runner::{ls, pwd}, models::Ls};
use std::path::PathBuf;

pub fn ls_manager(args: Vec<String>) {
    let mut all = false;
    let mut classify = false;
    let mut long = false;
    let mut invalid_input_found = false;
    let mut path_args_count = 0;
    let mut valid_paths: Vec<PathBuf> = Vec::new();

    for arg in args {
        if arg.starts_with('-') && arg != "-" {
            for char in arg.chars().skip(1) {
                if char == 'a' {
                    all = true;
                } else if char == 'l' {
                    long = true;
                } else if char == 'F' {
                    classify = true;
                } else {
                    println!("ls: invalid option -- '{}'", char);
                    return;
                }
            }
        } else {
            path_args_count += 1;
            let path = PathBuf::from(&arg);
            if path.exists() {
                valid_paths.push(path);
            } else {
                println!("ls: {}: No such file or directory", arg);
                invalid_input_found = true;
            }
        }
    }

    if valid_paths.is_empty() && !invalid_input_found {
        ls::run(Ls { all, long, classify, path: PathBuf::from(".") });
        return;
    }

    valid_paths.sort();

    let show_header = path_args_count > 1;

    for (i, path) in valid_paths.iter().enumerate() {
        if show_header {
            if i > 0 {
                println!(); 
            }
            println!("{}:", path.display());
        }
        ls::run(Ls { all, long, classify, path: path.clone() });
    }
}

pub fn pwd_manager() {
    pwd::run();
}
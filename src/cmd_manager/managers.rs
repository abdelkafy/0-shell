use crate::{
    cmd_runner::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm},
    models::{Ls, models::Flags},
};
use std::path::PathBuf;

pub fn ls_manager(args: Vec<String>) {
    let mut all = false;
    let mut classify = false;
    let mut long = false;
    let mut invalid_input_found = false;
    let mut path_args_count = 0;
    let mut valid_paths: Vec<PathBuf> = Vec::new();
    let mut valid_files: Vec<PathBuf> = Vec::new();
    let mut accept_flags = true;
    for arg in args {
        if arg == "--" {
            accept_flags = false;
        } else if accept_flags && arg.starts_with('-') && arg != "-" {
            for char in arg.chars().skip(1) {
                if char == 'a' {
                    all = true;
                } else if char == 'l' {
                    long = true;
                } else if char == 'F' {
                    classify = true;
                } else {
                    eprintln!("ls: invalid option -- '{}'", char);
                    return;
                }
            }
        } else {
            path_args_count += 1;

            let mut path = PathBuf::from(&arg);

            if arg.starts_with("~/") {
                let rest_of_path = arg[1..].trim_matches('/');

                if let Some(home) = dirs::home_dir() {
                    path = home.join(rest_of_path);
                }
                if let Some(home) = dirs::home_dir() {
                    path = home.join(rest_of_path);
                }
            }

            if path.exists() {
                if path.is_dir() {
                    valid_paths.push(path);
                } else {
                    valid_files.push(path);
                }
            } else {
                eprintln!("ls: {}: No such file or directory", arg);
                invalid_input_found = true;
            }
        }
    }
    let is_empty = valid_files.is_empty();
    ls::ls(
        valid_files,
        Flags {
            all,
            long,
            classify,
        },
        false,
    );

    if valid_paths.is_empty() && is_empty && !invalid_input_found {
        ls::run(Ls {
            flags: Flags {
                all,
                long,
                classify,
            },
            path: PathBuf::from("."),
        });
        return;
    }

    let show_header = path_args_count > 1;

    for (i, path) in valid_paths.iter().enumerate() {
        if show_header {
            if i > 0 {
                println!();
            }
            println!("{}:", path.display());
        }
        ls::run(Ls {
            flags: Flags {
                all,
                long,
                classify,
            },
            path: path.clone(),
        });
    }
}

pub fn pwd_manager() {
    pwd::run();
}
pub fn cat_manager(args: Vec<String>) {
    cat::run(args);
}
use crate::models::ShellPath;

pub fn cd_manager(args: Vec<String>,shell_path: &mut ShellPath) {
    cd::run(args, shell_path);
}

pub fn cp_manager(args: Vec<String>) {
    cp::run(args[0].as_str(), args[1].as_str());
}
pub fn echo_manager(args: Vec<String>) {
    echo::run(args);
}
pub fn mkdir_manager(args: Vec<String>) {
    mkdir::run(args);
}
pub fn rm_manager(args: Vec<String>) {
    rm::run(args);
}
pub fn mv_manager(args: Vec<String>) {
    mv::run(args);
}

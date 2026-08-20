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
            if arg=="~" {
                 if let Some(home) = dirs::home_dir() {
                    path = home;
                }
            }else if arg.starts_with("~/") {
                let rest_of_path = arg[1..].trim_matches('/');

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
    let files_is_empty = valid_files.is_empty();
    if !files_is_empty{
         ls::ls_files(
        valid_files
        ,false,
        Flags {
            all,
            long,
            classify,
        },
        
    );
    if !valid_paths.is_empty() {
        println!()
    }
    }
   
    if valid_paths.is_empty() && files_is_empty && !invalid_input_found {
        ls::run_dir(Ls {
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
        ls::run_dir(Ls {
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
    cp::run(args);
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

pub fn exit_manager(args: Vec<String>) {
    if args.is_empty() {
        std::process::exit(0);
    }

    match args[0].parse::<i64>() {
        Ok(n) => {
            if n > 0 {
                let status = n.rem_euclid(256) as i32;
                std::process::exit(status);
            } else {
                eprintln!("sh: exit: Illegal number: {}", args[0]);
            }
        }
        Err(_) => {
            eprintln!("sh: exit: Illegal number: {}", args[0]);
        }
    }
}
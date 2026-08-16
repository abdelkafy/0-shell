use crate::{cmd_runner::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm}, models::Ls};
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
pub fn cat_manager(args: Vec<String>){
    cat::run(args[0].as_str())
}
pub fn cd_manager(args: Vec<String>){
    cd::run(args[0].as_str());
}
pub fn cp_manager(args: Vec<String>){
   cp::run(args[0].as_str(),args[1].as_str());
}
pub fn echo_manager(args: Vec<String>){
    echo::run(args);
}
pub fn mkdir_manager(args: Vec<String>){
    mkdir::run(args);
}
pub fn rm_manager(args: Vec<String>){
    rm::run(args);
}
pub fn mv_manager(args: Vec<String>){
    mv::run(args);
}

use std::env;

pub fn run() {


    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }

        Err(err) => {
            eprintln!("pwd: {}", crate::errors::format_error(&err));
        }
    }
}
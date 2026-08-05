use std::env;

pub fn run(path: &str) {

    match env::set_current_dir(path) {
        Ok(_) => {
            println!("Current dir: {}", env::current_dir().unwrap().display());
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
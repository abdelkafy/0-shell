use std::fs;

pub fn run(name: &str) {
    match fs::create_dir(name) {
        Ok(_) => {}
        Err(err) => eprintln!("mkdir: {}", err),
    }
}
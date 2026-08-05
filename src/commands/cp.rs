use std::fs;

pub fn run(source: &str, destination: &str) {
    match fs::copy(source, destination) {
        Ok(_) => {}
        Err(err) => eprintln!("cp: {}", err),
    }
}
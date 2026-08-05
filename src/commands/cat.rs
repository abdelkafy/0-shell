use std::fs;

pub fn run(file: &str) {
    match fs::read_to_string(file) {
        Ok(content) => print!("{}", content),
        Err(err) => eprintln!("cat: {}", err),
    }
}
use std::env;
use crate::models;

pub fn run(cmd : models::Ls) {
  let current_path= env::current_dir().unwrap();
  let entries = std::fs::read_dir(&current_path).unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    let file_name = entry.file_name();
    println!("{}", file_name.to_string_lossy());
  }
}


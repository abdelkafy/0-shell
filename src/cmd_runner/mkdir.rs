use std::fs;
use crate::errors::format_error;

pub fn run(names: Vec<String>) {
    if names.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for name in names {
        let name = name.replace('\n', "\\n");

        if let Err(err) = fs::create_dir(&name) {
            eprintln!(
                "mkdir: cannot create directory '{}': {}",
                name,
                format_error(&err)
            );
        }
    }
}
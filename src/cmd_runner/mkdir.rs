use std::fs;

pub fn run(names: Vec<String>) {
    if names.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    let mut parse_flags = true;
    let mut targets = Vec::new();

    for name in names {
        if parse_flags {
            if name == "--" {
                parse_flags = false;
                continue;
            }

            if name.starts_with('-') {
                let flag = name.chars().nth(1).unwrap_or('-');

                eprintln!("mkdir: invalid option -- '{}'", flag);
                continue;
            }
        }

        targets.push(name);
    }

    if targets.is_empty() {
        return;
    }

    for name in targets {
        let name = name.replace('\n', "\\n");

        if let Err(_) = fs::create_dir(&name) {
            eprintln!(
                "mkdir: cannot create directory '{}'",
                name,
            );
        }
    }
}
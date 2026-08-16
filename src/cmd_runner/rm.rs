use std::fs;
use std::path::Path;

pub fn run(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    let recursive = args.contains(&"-r".to_string());

    let targets: Vec<&String> = args
        .iter()
        .filter(|arg| arg.as_str() != "-r")
        .collect();

    for target in targets {
        let path = Path::new(target);

        if !path.exists() {
            eprintln!("rm: {}: No such file or directory", target);
            continue;
        }

        if path.is_dir() {
            if recursive {
                if let Err(err) = fs::remove_dir_all(path) {
                    eprintln!("rm: {}: {}", target, err);
                }
            } else {
                eprintln!("rm: {}: is a directory", target);
            }
        } else {
            if let Err(err) = fs::remove_file(path) {
                eprintln!("rm: {}: {}", target, err);
            }
        }
    }
}
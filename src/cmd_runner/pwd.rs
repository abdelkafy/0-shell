use std::env;

pub fn run(args: Vec<String>) {

    if !args.is_empty() {
        eprintln!("pwd: too many arguments");
        return;
    }


    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }

        Err(err) => {
            eprintln!("pwd: {}", crate::errors::format_error(&err));
        }
    }
}
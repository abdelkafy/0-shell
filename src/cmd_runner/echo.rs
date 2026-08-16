pub fn run(args: Vec<String>) {
    let args: Vec<String> = args
        .iter()
        .map(|arg| {
            let without_quotes = remove_quotes(arg);
            handle_escape(without_quotes)
        })
        .collect();

    println!("{}", args.join(" "));
}

fn remove_quotes(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') {
        return s[1..s.len()-1].to_string();
    }

    s.to_string()
}

fn handle_escape(s: String) -> String {
    s.replace("\\n", "\n")
}
pub fn run(args: Vec<String>) {
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| handle_escape(arg))
        .collect();

    println!("{}", args.join(" "));
}

fn handle_escape(s: String) -> String {
    s.replace("\\n", "\n")
}

use std::fs;
use std::io;
use std::path::Path;

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("mv: missing operand");
        return;
    }

    let source = &args[0];
    let destination = &args[1];

    let final_destination = if Path::new(destination).is_dir() {
        let filename = Path::new(source)
            .file_name()
            .unwrap();

        Path::new(destination).join(filename)
    } else {
        Path::new(destination).to_path_buf()
    };

    match fs::rename(source, &final_destination) {
        Ok(_) => {}

        Err(err) => {
            // cross filesystem
            if err.raw_os_error() == Some(18) {
                match copy_and_remove(source, &final_destination) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("mv: {}", e);
                    }
                }
            } else {
                eprintln!(
                    "mv: {} -> {}: {}",
                    source,
                    final_destination.display(),
                    err
                );
            }
        }
    }
}

fn copy_and_remove(source: &str, destination: &Path) -> io::Result<()> {
    fs::copy(source, destination)?;
    fs::remove_file(source)?;

    Ok(())
}
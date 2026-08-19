use std::io;

pub fn format_error(err: &io::Error) -> &str {
    match err.kind() {
        io::ErrorKind::NotFound => "No such file or directory",
        io::ErrorKind::PermissionDenied => "Permission denied",
        io::ErrorKind::IsADirectory => "Is a directory",
        io::ErrorKind::AlreadyExists => "File exists",
        io::ErrorKind::NotADirectory => "Not a directory",
        io::ErrorKind::InvalidInput => "Invalid argument",
        io::ErrorKind::InvalidData => "Invalid data",
        io::ErrorKind::BrokenPipe => "Broken pipe",
        io::ErrorKind::WouldBlock => "Resource temporarily unavailable",
        io::ErrorKind::TimedOut => "Operation timed out",
        io::ErrorKind::WriteZero => "Write zero",
        io::ErrorKind::Interrupted => "Interrupted system call",
        io::ErrorKind::UnexpectedEof => "Unexpected end of file",
        _ => "Input/output error",
    }
}
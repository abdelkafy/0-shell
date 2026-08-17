use std::{
    fs::Metadata,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};
use crate::models;
use uzers::{get_group_by_gid, get_user_by_uid};
use std::time::{SystemTime, UNIX_EPOCH};
use jiff::{SignedDuration, Timestamp};
use jiff::tz::TimeZone;
struct File<'a> {
    file: &'a Path,
    formatted_output: String,
}

pub fn run(cmd: models::Ls) {
    let entries = match std::fs::read_dir(&cmd.path) {
        Ok(read_dir) => read_dir,
        Err(_) => return,
    };

    let unfiltered_files = entries.filter_map(|entry| entry.ok().map(|e| e.path()));
    let mut files: Vec<PathBuf> = Vec::new();

    if !cmd.all {
        files = unfiltered_files
            .filter(|file| {
                file.file_name()
                    .map(|name| !name.to_string_lossy().starts_with('.'))
                    .unwrap_or(true)
            })
            .collect();
    } else {
        files.push(PathBuf::from("."));
        files.push(PathBuf::from(".."));
        files.extend(unfiltered_files);
    }

    let mut formatted: Vec<File> = if cmd.long {
         let size_width = std::cmp::max(
     8,
     files
        .iter()
        .filter_map(|path| path.symlink_metadata().ok())
        .map(|metadata| metadata.len().to_string().len())
        .max()
        .unwrap_or(1),
        );
        files
            .iter()
            .map(|path| File {
                file: path,
                formatted_output: long_format(path,size_width),
            })
            .collect()
    } else {
        files
            .iter()
            .map(|path| File {
                file: path,
                formatted_output: normal_format(path),
            })
            .collect()
    };

    if cmd.classify {
        for file in &mut formatted {
            file.formatted_output.push_str(&classify(file.file,cmd.long));
        }
    }

    formatted.sort_by_key(|file: &File<'_>| file.file.to_string_lossy());

    let separator = if cmd.long { "\n" } else { " " };

    for file in formatted {
        print!("{}{}", file.formatted_output, separator);
    }
    println!();
}

fn long_format(path: &Path,size_width:usize) -> String {
    let metadata = match path.symlink_metadata() {
        Ok(meta) => meta,
        Err(_) => return path.to_string_lossy().into_owned(),
    };

    let permissions = metadata.permissions();

    let type_char = if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    };

    let mode = permissions.mode();
    let perm_str = format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0b100_000_000 != 0 { 'r' } else { '-' },
        if mode & 0b010_000_000 != 0 { 'w' } else { '-' },
        if mode & 0b001_000_000 != 0 { 'x' } else { '-' },
        if mode & 0b000_100_000 != 0 { 'r' } else { '-' },
        if mode & 0b000_010_000 != 0 { 'w' } else { '-' },
        if mode & 0b000_001_000 != 0 { 'x' } else { '-' },
        if mode & 0b000_000_100 != 0 { 'r' } else { '-' },
        if mode & 0b000_000_010 != 0 { 'w' } else { '-' },
        if mode & 0b000_000_001 != 0 { 'x' } else { '-' },
    );

    let hard_links_pointing = metadata.nlink();

    let owner_name = match get_user_by_uid(metadata.uid()) {
        Some(user) => user.name().to_string_lossy().into_owned(),
        None => metadata.uid().to_string(),
    };

    let group_name = match get_group_by_gid(metadata.gid()) {
        Some(group) => group.name().to_string_lossy().into_owned(),
        None => metadata.gid().to_string(),
    };

    let size = metadata.len();

   let modified_at = metadata.modified().unwrap_or(UNIX_EPOCH);

    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| path.to_string_lossy());
   
    format!(
        "{}{} {} {} {} {:>size_width$} {} {}",
        type_char,
        perm_str,
        hard_links_pointing,
        owner_name,
        group_name,
        size,
        format_time(modified_at),
        file_name
    )
}

fn normal_format(path: &Path) -> String {
    match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().into_owned(),
        None => ".".to_string(),
    }
}
fn classify(file_path: &Path, long: bool) -> String {
    let metadata = match file_path.symlink_metadata() {
        Ok(meta) => meta,
        Err(_) => return String::new(),
    };

    let symbol = if metadata.file_type().is_dir() {
        "/".to_string()
    } else if metadata.file_type().is_fifo() {
        "|".to_string()
    } else if metadata.file_type().is_symlink() {
        if long {
            match file_path.read_link() {
                Ok(target) => format!(" -> {}", target.display()),
                Err(_) => " -> ?".to_string(),
            }
        } else {
            "@".to_string()
        }
    } else if metadata.file_type().is_socket() {
        "=".to_string()
    } else if is_door(&metadata) {
        ">".to_string()
    } else if is_exe(&metadata) {
        "*".to_string()
    } else {
        String::new()
    };

    symbol
}
fn is_door(entry: &Metadata) -> bool {
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        let mode = entry.mode();
        (mode & libc::S_IFMT) == libc::S_IFDOOR
    }

    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
    {
        let _ = entry;
        false
    }
}

fn is_exe(entry: &Metadata) -> bool {
    #[cfg(unix)]
    {
        entry.is_file() && (entry.permissions().mode() & 0o111) != 0
    }

    #[cfg(not(unix))]
    {
        let _ = entry;
        false
    }
}

fn format_time(time: SystemTime) -> String {
    let file_time = match Timestamp::try_from(time) {
        Ok(timestamp) => timestamp,
        Err(_) => Timestamp::UNIX_EPOCH,
    };

    let now = Timestamp::now();
    let age = file_time.duration_until(now);

    let six_months = SignedDuration::from_secs(180 * 24 * 60 * 60);

    let local = file_time.to_zoned(TimeZone::system());

    if age >= SignedDuration::ZERO && age < six_months {
        local.strftime("%b %e %H:%M").to_string()
    } else {
        local.strftime("%b %e  %Y").to_string()
    }
}
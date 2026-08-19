use crate::models::models::Flags;
use crate::models::{self};
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fs::Metadata,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};
use uzers::{get_group_by_gid, get_user_by_uid};

#[derive(Clone, Copy)]
struct MaxSizes {
    max_size_width: usize,
    max_links_width: usize,
    max_owner_width: usize,
    max_group_width: usize,
}

struct File<'a> {
    file: &'a Path,
    formatted_output: String,
}

pub fn run_dir(cmd: models::Ls) {
    let entries = match std::fs::read_dir(&cmd.path) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            eprintln!("ls: dir:{}", err);
            return;
        }
    };
    let unfiltered_files = entries.filter_map(|entry| entry.ok().map(|e| e.path()));
    let mut files: Vec<PathBuf> = Vec::new();

    if !cmd.flags.all {
        files = unfiltered_files
            .filter(|file| {
                file.file_name()
                    .map(|name| !name.to_string_lossy().starts_with('.'))
                    .unwrap_or(true)
            })
            .collect();
    } else {
        files.push(cmd.path.join("."));
        files.push(cmd.path.join(".."));
        files.extend(unfiltered_files);
    }
    let total_blocks: u64 = files
        .iter()
        .filter_map(|path| path.symlink_metadata().ok())
        .map(|meta| {
            let blocks = meta.blocks() as u64;
            (blocks / 2).max(0)
        })
        .sum();

    if cmd.flags.long {
        println!("total {}", total_blocks);
    }
    ls_files(files, cmd.flags, true);
}

pub fn ls_files(files: Vec<PathBuf>, cmd_flags: Flags, is_dir: bool) {
    let mut formatted: Vec<File> = if cmd_flags.long {
        let max_size_width = std::cmp::max(
            8,
            files
                .iter()
                .filter_map(|path| path.symlink_metadata().ok())
                .map(|metadata| metadata.len().to_string().len())
                .max()
                .unwrap_or(1),
        );
        let max_links_width = std::cmp::max(
            3,
            files
                .iter()
                .filter_map(|p| p.symlink_metadata().ok())
                .map(|m| m.nlink().to_string().len())
                .max()
                .unwrap_or(1),
        );

        let max_owner_width = std::cmp::max(
            8,
            files
                .iter()
                .filter_map(|p| p.symlink_metadata().ok())
                .map(|m| {
                    get_user_by_uid(m.uid())
                        .map(|u| u.name().to_string_lossy().len())
                        .unwrap_or_else(|| m.uid().to_string().len())
                })
                .max()
                .unwrap_or(1),
        );

        let max_group_width = std::cmp::max(
            8,
            files
                .iter()
                .filter_map(|p| p.symlink_metadata().ok())
                .map(|m| {
                    get_group_by_gid(m.gid())
                        .map(|g| g.name().to_string_lossy().len())
                        .unwrap_or_else(|| m.gid().to_string().len())
                })
                .max()
                .unwrap_or(1),
        );
        files
            .iter()
            .map(| path| {
                let virtual_path = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                File {
                    file: path,
                    formatted_output: long_format(
                        path,
                        virtual_path,
                        MaxSizes {
                            max_size_width,
                            max_links_width,
                            max_owner_width,
                            max_group_width,
                        },
                        cmd_flags.classify,
                    ),
                }
            })
            .collect()
    } else {
        files
            .iter()
            .map(|path| {
                let mut virtual_path = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                File {
                    file: path,
                    formatted_output: {
                        if cmd_flags.classify {
                            virtual_path.push_str(&classify(path, false));
                        }
                        virtual_path
                    },
                }
            })
            .collect()
    };

    formatted.sort_by_key(|file: &File<'_>| {
        let file_path = file.file.to_string_lossy();
        let splitted: Vec<&str> = file_path.split("/").collect();
        let len = splitted.len();
        if len >= 1 {
            return splitted[len - 1].to_owned();
        } else {
            return "".to_owned();
        }
    });

    for file in formatted {
        if cmd_flags.long {
            println!("{}", file.formatted_output);
        } else {
            print!("{}  ", file.formatted_output);
        }
    }
    if !cmd_flags.long {
        println!();
    }
}

fn long_format(
    path: &Path,
    virtual_path: String,
    max_sizes: MaxSizes,
    is_classify: bool,
) -> String {
    let metadata = match path.symlink_metadata() {
        Ok(meta) => meta,
        Err(_) => return path.to_string_lossy().into_owned(),
    };

    let permissions = metadata.permissions();

    let type_char = {
        let ft = metadata.file_type();
        if ft.is_dir() {
            'd'
        } else if ft.is_symlink() {
            'l'
        } else if ft.is_fifo() {
            'p'
        } else if ft.is_socket() {
            's'
        } else if ft.is_char_device() {
            'c'
        } else if ft.is_block_device() {
            'b'
        } else {
            '-'
        }
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

    let size_or_device =
        if metadata.file_type().is_char_device() || metadata.file_type().is_block_device() {
            let rdev = metadata.rdev();
            let major = libc::major(rdev);
            let minor = libc::minor(rdev);
            format!("{:>3}, {:>3}", major, minor)
        } else {
            let max_width = max_sizes.max_size_width;
            format!("{:>max_width$}", metadata.len())
        };

    let modified_at = metadata.modified().unwrap_or(UNIX_EPOCH);

    let max_links_width = max_sizes.max_links_width;
    let max_owner_width = max_sizes.max_owner_width;
    let max_group_width = max_sizes.max_group_width;
    let suffix = if metadata.file_type().is_symlink() {
        classify(path, true)
    } else if is_classify {
        classify(path, false)
    } else {
        String::new()
    };
    format!(
        "{}{} {:>max_links_width$} {:<max_owner_width$} {:<max_group_width$} {} {} {}{}",
        type_char,
        perm_str,
        hard_links_pointing,
        owner_name,
        group_name,
        size_or_device,
        format_time(modified_at),
        virtual_path,
        suffix,
    )
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
    } else if is_exe(&metadata) {
        "*".to_string()
    } else {
        String::new()
    };

    symbol
}

fn is_exe(entry: &Metadata) -> bool {
    entry.is_file() && (entry.permissions().mode() & 0o111) != 0
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

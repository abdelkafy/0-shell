use std::{  fs::{DirEntry, Metadata}, os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt}, path::{self, Path, PathBuf}};
use crate::models;
use uzers::{get_user_by_uid, get_group_by_gid};
use std::time::UNIX_EPOCH;
use std::env;
use chrono::{DateTime, Local, TimeZone, Datelike};

struct File <'a> {
  file:&'a Path,
  formatted_output: String,
}

pub fn run(cmd : models::Ls) {
  let entries = std::fs::read_dir(&cmd.path).unwrap();
  let unfiltered_files = entries.map(|entry|entry.unwrap().path());
  let mut files=Vec::new();
  if !(cmd.all){
    // i must change to_str cause it break when emoji
    files=unfiltered_files.filter(|file|file.file_name().unwrap().to_string_lossy().starts_with(".")).collect();
  }else{
    files=unfiltered_files.collect();
    let current_dir = PathBuf::from(".");
    let parent_dir = PathBuf::from("..");
    files.insert(0, current_dir);
    files.insert(1, parent_dir);

  }
  let mut formatted=Vec::new();
  if cmd.long{
    formatted=files.iter().map(|path|{
        
      File{
        file:&path,
        formatted_output:long_format(&path),
      }
    }).collect();
  }else{
    formatted=files.iter().map(|path|{
      File{
        file:&path,
        formatted_output:normal_format(&path),
      }
    }).collect();
  }
  if cmd.classify {
      formatted=formatted.iter().map(|file|{
        let mut formatted_output=(&file.formatted_output).to_string();
        formatted_output.push_str(&classify(file.file));
        File { file: file.file, formatted_output: formatted_output.to_string() }
      }).collect();
  }
  let mut separetor=" ";
  if cmd.long{
   separetor="\n";

  }
  for file in formatted{
    print!("{}{}",file.formatted_output,separetor);
  }
  println!()
}
fn long_format(path: &Path) -> String {
    let metadata = path.symlink_metadata().unwrap();
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

    let modified_at = metadata
        .modified()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let file_name = path
    .file_name()
    .map(|s| s.to_string_lossy())
    .unwrap_or_else(|| path.to_string_lossy());

    format!(
        "{}{} {} {} {} {:>8} {} {}",
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
fn normal_format(path: &Path)->String{
    println!("{:?}",path);
    match path.file_name(){
        Some(file_name)=>file_name.to_string_lossy().into_owned(),
        None=>".".to_string()
    }
    
    
}
fn classify(file_path : &Path)-> String{
  let mut symbole="";
  let metadata=file_path.symlink_metadata().unwrap();
  if metadata.file_type().is_dir() {
      symbole="/";
  }else if metadata.file_type().is_fifo(){
      symbole="|";
      
  }else if metadata.file_type().is_symlink() {
      symbole="@";
      
  }else if metadata.file_type().is_socket() {
      symbole="=";
      
  }else if is_door(&metadata) {
      symbole=">";
      
  }else if is_exe(&metadata){
      symbole="*";
      
  }
 symbole.to_string()
}
fn is_door(entry: &Metadata) -> bool {
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        let mode = entry.mode();
        (mode & libc::S_IFMT) == libc::S_IFDOOR
    }

    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
    {
        let _ = entry; // Silence unused variable warning on other OSes
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
fn format_time(seconds: u64) -> String {
    let now = Local::now();
    let file_time = Local.timestamp_opt(seconds as i64, 0)
        .single()
        .unwrap_or_else(|| Local.timestamp_opt(0, 0).single().unwrap());

    let six_months_in_seconds = 6 * 30 * 24 * 60 * 60; 
    let time_difference = now.timestamp() - file_time.timestamp();

    if time_difference < six_months_in_seconds && time_difference >= 0 {
        file_time.format("%b %e %H:%M").to_string()
    } else {
        file_time.format("%b %e  %Y").to_string()
    }
}

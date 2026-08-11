use std::{ env, fs::DirEntry, os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt}};
use crate::models;
use uzers::{get_user_by_uid, get_group_by_gid};
use std::time::UNIX_EPOCH;

struct File <'a> {
  file:&'a DirEntry,
  formatted_output: String,
}

pub fn run(cmd : models::Ls) {
  let entries = std::fs::read_dir(&cmd.path).unwrap();
  let unfiltered_files = entries.map(|entry|entry.unwrap());
  let mut files=Vec::new();
  if !(cmd.all){
    // i must change to_str cause it break when emoji
    files=unfiltered_files.filter(|file|file.file_name().to_str().unwrap().starts_with(".")).collect();
  }else{
    files=unfiltered_files.collect();
  }
  let mut formatted=Vec::new();
  if cmd.long{
    formatted=files.iter().map(|file|{
      File{
        file:file,
        formatted_output:long_format(file),
      }
    }).collect();
  }else{
    formatted=files.iter().map(|file|{
      File{
        file:file,
        formatted_output:normal_format(file),
      }
    }).collect();
  }
  if cmd.classify {
      formatted=formatted.iter().map(|file|{
        let mut formatted_output=(&file.formatted_output).to_string();
        formatted_output.push_str(&classify(file));
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
fn long_format(file: &DirEntry) -> String {
    let metadata = file.metadata().unwrap();
    let file_type = file.file_type().unwrap();
    let permissions = metadata.permissions();

    let type_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
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

    let file_name = file.file_name().to_string_lossy().into_owned();

    format!(
        "{}{} {} {} {} {:>8} {} {}",
        type_char,
        perm_str,
        hard_links_pointing,
        owner_name,
        group_name,
        size,
        modified_at,
        file_name
    )
}
fn normal_format(file: &DirEntry)->String{
    file.file_name().to_string_lossy().into_owned()
}
fn classify(file : &File)-> String{
  let mut symbole="";
  if file.file.file_type().unwrap().is_dir() {
      symbole="/";
  }else if file.file.file_type().unwrap().is_fifo(){
      symbole="|";
      
  }else if file.file.file_type().unwrap().is_symlink() {
      symbole="@";
      
  }else if file.file.file_type().unwrap().is_socket() {
      symbole="=";
      
  }else if is_door(file.file) {
      symbole=">";
      
  }else if is_exe(file.file){
      symbole="*";
      
  }
 symbole.to_string()
}
fn is_door(entry: &DirEntry) -> bool {
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        if let Ok(metadata) = entry.metadata() {
            let mode = metadata.mode();
            return (mode & libc::S_IFMT) == libc::S_IFDOOR;
        }
    }
    
    false 
}
fn is_exe(entry: &DirEntry) -> bool {
    entry.metadata().map_or(false, |meta| {
        meta.is_file() && (meta.permissions().mode() & 0o111) != 0
    })
}
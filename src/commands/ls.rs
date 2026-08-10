use std::{any::Any, env, fs::DirEntry, os::unix::fs::MetadataExt};
use crate::models;
use uzers::{get_user_by_uid, get_group_by_gid};

pub fn run(cmd : models::Ls) {
  let current_path= env::current_dir().unwrap();
  let entries = std::fs::read_dir(&current_path).unwrap();
  let unfiltered_files = entries.map(|entry|entry.unwrap());
  let mut files=Vec::new();
  if !(cmd.all){
    // i must change to_str cause it break when emoji
    files=unfiltered_files.filter(|file|file.file_name().to_str().unwrap().starts_with(".")).collect();
  }else{
    files=unfiltered_files.collect();
  }
  let mut long_formatted:Vec<String>=Vec::new();
  if cmd.long{

  }
  if  cmd.classify {
      
  }
}
fn format(file:DirEntry)->String{
  let mut  long_format=String::new();

  let metadata=file.metadata().unwrap();
  
  let file_type=file.file_type();
  
  let permissions=metadata.permissions();

  let modified_at=metadata.modified();
  let owner_name= match get_user_by_uid(metadata.uid()){
    Some(user)=>user.name().to_string_lossy().into_owned(),
    None=>metadata.uid().to_string()
  };
  let group_name= match get_group_by_gid(metadata.gid()){
    Some(group)=>group.name().to_string_lossy().into_owned(),
    None=>metadata.gid().to_string()
  };
  let hard_links_pointing=metadata.nlink();

} 
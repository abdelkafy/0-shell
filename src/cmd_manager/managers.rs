use crate::{cmd_runner::{ls, pwd}, models::Ls};
use std::path::{Path, PathBuf};

pub fn ls_manager(args :Vec<String>){
    let mut all =false;
    let mut classify=false;
    let mut  long=false;
    let mut invalid_input_found=false;
    let mut  valid_paths:Vec<PathBuf> = Vec::new();
    for arg in args {
        if arg.starts_with("-"){
            for char in arg.chars().skip(1){
                if char=='a'{
                    all=true;
                
                }else if char=='l' {
                    long=true;

                }else if char=='F'{
                    classify=true;

                }else{
                    println!("invalid")
                }
            }
        }else{
                let path = PathBuf::from(arg);
                if path.exists(){
                    valid_paths.push(path);
                }else{
                    invalid_input_found=true;
                    println!("no")
                }
        }
    }
   if valid_paths.len()==0 && !invalid_input_found {
    ls::run(Ls { all, long, classify, path:PathBuf::from(".") });

   }
    valid_paths.sort();
    for path in valid_paths{
        println!("{}:",path.display());
        ls::run(Ls { all, long, classify, path });
        println!()
    }
}
pub fn pwd_manager(){
    pwd::run();
}
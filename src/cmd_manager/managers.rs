use crate::{cmd_runner::ls, models::Ls};
use std::path::{Path, PathBuf};

pub fn ls_manager(args :Vec<String>){
    let mut all =false;
    let mut classify=false;
    let mut  long=false;
    let mut  valid_paths:Vec<PathBuf> = Vec::new();
    for arg in args {
        if arg.starts_with("-"){
            for char in arg.chars(){
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
                    println!("no")
                }
        }
    }
    for path in valid_paths{
        ls::run(Ls { all, long, classify, path });
    }
}
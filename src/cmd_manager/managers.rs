use crate::models::{Ls};
pub fn ls_manager(args :Vec<String>){
    let mut all =false;
    let mut classify=false;
    let mut  long=false;
    let mut  valid_paths:Vec<String> = Vec::new();
    for arg in args {
        if arg.starts_with("-"){
            if arg.contains("a"){
                all=true;
            }
            if arg.contains("l"){
                long=true;
            }
             if arg.contains("F"){
                classify=true;
            }
        }else{
            if {

            }
        }
    }
}
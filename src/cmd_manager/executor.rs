use crate::{cmd_manager::managers::ls_manager, models::Command};


pub fn command_executor(command:Command,args :Vec<String>){
    match command {
        Command::Ls=>ls_manager(args),
        _=>print!("others"),
    }
}
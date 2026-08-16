use crate::{cmd_manager::managers::{ls_manager, pwd_manager}, models::Command};


pub fn command_executor(command:Command,args :Vec<String>){
    match command {
        Command::Ls=>ls_manager(args),
        Command::Pwd=>pwd_manager(),
        _=>print!("others"),
    }
}
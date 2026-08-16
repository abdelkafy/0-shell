use crate::{cmd_manager::managers , models::Command};


pub fn command_executor(command:Command,args :Vec<String>){
    match command {
        Command::Ls=>managers::ls_manager(args),
        Command::Pwd=>managers::pwd_manager(),
        Command::Cat=>managers::cat_manager(args),
        Command::Cp=>managers::cp_manager(args),
        Command::Cd=>managers::cd_manager(args),
        Command::Echo=>managers::echo_manager(args),
        Command::Mkdir=>managers::mkdir_manager(args),
        Command::Mv=>managers::mv_manager(args),
        Command::Rm=>managers::rm_manager(args),
        _=>print!("others"),
    }
}
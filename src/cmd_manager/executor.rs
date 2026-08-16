use crate::{cmd_manager::managers::{cat_manager, cd_manager, cp_manager, echo_manager, ls_manager, mkdir_manager, mv_manager, pwd_manager, rm_manager}, cmd_runner::mv, models::Command};


pub fn command_executor(command:Command,args :Vec<String>){
    match command {
        Command::Ls=>ls_manager(args),
        Command::Pwd=>pwd_manager(),
        Command::Cat=>cat_manager(args),
        Command::Cp=>cp_manager(args),
        Command::Cd=>cd_manager(args),
        Command::Echo=>echo_manager(args),
        Command::Mkdir=>mkdir_manager(args),
        Command::Mv=>mv_manager(args),
        Command::Rm=>rm_manager(args),
        _=>print!("others"),
    }
}
use crate::models::{Command, ShellPath};
use super::managers;

pub fn command_executor(
    command: Command,
    args: Vec<String>,
    shell_path: &mut ShellPath,
) {
    match command {
        Command::Cd => managers::cd_manager(args, shell_path),
        Command::Pwd => managers::pwd_manager(),
        Command::Ls => managers::ls_manager(args),
        Command::Cat => managers::cat_manager(args),
        Command::Cp => managers::cp_manager(args),
        Command::Mv => managers::mv_manager(args),
        Command::Rm => managers::rm_manager(args),
        Command::Mkdir => managers::mkdir_manager(args),
        Command::Echo => managers::echo_manager(args),
        Command::Exit => managers::exit_manager(args),
    }
}

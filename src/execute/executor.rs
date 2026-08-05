use crate::models::Command;

pub fn execute(command: Command) {
    match command {
        Command::Pwd => crate::commands::pwd::run(),
        Command::Ls(ls) => crate::commands::ls::run(ls),
        _ => {}
    }
}
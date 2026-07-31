use crate::models::Command;

pub fn execute(command: Command) {
    match command {
        Command::Pwd => crate::commands::pwd::run(),
        _ => {}
    }
}
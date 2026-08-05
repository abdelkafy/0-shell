use crate::models::Command;
use crate::commands::cd;
use crate::commands::mkdir;
use crate::commands::cat;
use crate::commands::echo;
use crate::commands::cp;

pub fn execute(command: Command) {
    match command {
        Command::Pwd => crate::commands::pwd::run(),
        Command::Cd(path) => cd::run(&path),
        Command::Mkdir(name) => mkdir::run(&name),
        Command::Cat(file) => cat::run(&file),
        Command::Echo(args) => echo::run(args),
        Command::Cp(source, destination) => {
        cp::run(&source, &destination)}
        _ => {}
    }
}
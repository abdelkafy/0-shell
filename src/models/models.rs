use std::path::PathBuf;

pub struct Ls {
    pub flags: Flags,
    pub path: PathBuf,
}

pub struct Flags {
    pub all: bool,
    pub long: bool,
    pub classify: bool,
}

pub enum Command {
    Echo,
    Cd,
    Ls,
    Pwd,
    Cat,
    Cp,
    Rm,
    Mv,
    Mkdir,
    Exit,
}


pub struct ShellPath {
    pub current: PathBuf,
    pub previous: Option<PathBuf>,
}

impl ShellPath {
    pub fn new() -> Self {
        let current = std::env::current_dir().unwrap();

        Self {
            current: current.clone(),
            previous: Some(current),
        }
    }
}
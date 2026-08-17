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

use std::path::PathBuf;


pub struct Ls{
    pub all: bool,
    pub long: bool,
    pub classify: bool,
    pub path: PathBuf,
}

//pub enum Rmflag{
//    Recursive,
//    Normal
//}

pub enum Command{
    Echo,
    Cd,
    Ls (Ls),
    Pwd,
    Cat,
    Cp,
    Rm ,
    Mv,
    Mkdir,
    Exit,
}
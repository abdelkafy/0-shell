pub enum Lsflag{
    All,
    Long,
    Classify,
    Normal,
}

//pub enum Rmflag{
//    Recursive,
//    Normal
//}

pub enum Command{
    Echo(Vec<String>),
    Cd(String),
    Ls (Lsflag) ,
    Pwd,
    Cat(String),
    Cp(String, String),
    Rm(Vec<String>),
    Mv(Vec<String>),
    Mkdir(Vec<String>),
    Exit,
}
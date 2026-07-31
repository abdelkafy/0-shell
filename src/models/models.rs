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
    Echo,
    Cd,
    Ls (Lsflag) ,
    Pwd,
    Cat,
    Cp,
    Rm ,
    Mv,
    Mkdir,
    Exit,
}
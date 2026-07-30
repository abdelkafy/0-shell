pub enum LsFlag{
    all,
    long,
    classify,
     normal,
}
pub enum RmFlag{
    recursive,
    normal
}
pub enum Command{
    echo,
cd,
ls (LsFlag) ,
pwd,
cat,
cp,
rm ,
mv,
mkdir,
exit,
}
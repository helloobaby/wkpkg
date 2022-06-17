use std::string::String;
use std::path;
use std::str::FromStr;
use std::ops::Deref;
use std::io::Result;




///
///[参数1] 存储包的git路径
/// 
pub fn install(path : &String) -> Result<()>{
    println!("[T]install");

    //校验路径是否合法(后缀必须是.git)
    let git_path = &path::PathBuf::from_str(path).unwrap();

    if git_path.extension().unwrap() != "git" {
        panic!("[E]error path suffix : {:?}",git_path.extension().unwrap());
    }

    //从git仓库下载包(实际上需要实现git clone这个功能)














    Ok(())
}
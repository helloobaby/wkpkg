
use std::vec;
use std::string;
use std::io::Result;
use std::env;
use std::panic;

mod install;

fn main() -> Result<()>{

    // 获得本程序的绝对路径
    let cexe = env::current_exe();
    let mut cmd_iter = env::args();

    println!("exe path : {}",cexe.as_ref().unwrap().display());

    let cexe =cexe.unwrap();
    let mut cexe = cexe.into_os_string().into_string().unwrap();

    let pos = cexe.rfind('\\').expect("invalid path");

    // 获得存储包的路径

    cexe.replace_range(pos+1.., "pkgs");
    println!("pkgs path : {}",cexe);

    // 处理命令行第二个参数
    cmd_iter.next();
    let cmd2 = cmd_iter.next().expect("[E]Error usage , See --help");
    println!("param2 : {}",cmd2);

    // 处理第三个参数
    let cmd3 = cmd_iter.next().expect("[E]Error usage , See --help");
    println!("param3 : {}",cmd3);

    // 
    match cmd2.as_str(){
        "install" => install::install(&cmd3),
        _ => panic!("[E]Error usage , See --help"),
    };






    Ok(())
}

#[warn(unused_must_use)]
use std::vec;
use std::string;
use std::io::Result;
use std::env;
use std::panic;

mod install;
mod help;

fn main() -> Result<()>{

    // 获得本程序的绝对路径
    let cexe = env::current_exe();
    let mut cmd_iter = env::args();

    println!("[I]exe path : {}",cexe.as_ref().unwrap().display());

    let cexe =cexe.unwrap();
    let mut cexe = cexe.into_os_string().into_string().unwrap();

    let pos = cexe.rfind('\\').expect("invalid path");

    // 获得存储包的路径

    cexe.replace_range(pos+1.., "pkgs");
    println!("[I]pkgs path : {}",cexe);

    // 处理命令行第二个参数
    cmd_iter.next();
    let cmd2 = cmd_iter.next().expect("[E]Error usage , See --help");
    println!("[I]param2 : {}",cmd2);


    //处理每个命令
    let err = match cmd2.as_str(){

        "install" => {
            // 处理第三个参数
            let cmd3 = cmd_iter.next().expect("[E]Error usage , See --help");
            println!("[I]param3 : {}",cmd3);

            // 处理第四个参数
            let cmd4 = cmd_iter.next().expect("[E]Error usage , See --help");
            println!("[I]param3 : {}",cmd4);
            install::install(&cexe,&cmd3,&cmd4)},



        "help" => help::help(),





        _ => panic!("[E]Error usage , See --help"),
    };
    return err;
}

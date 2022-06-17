use std::string::String;
use std::path;
use std::str::FromStr;
use std::ops::Deref;
use std::io::Result;
use std::process::Command;
use std::io::{self, Write};
use std::fs;

mod fix;
mod cmd;


///
///[参数1] 本地存放包的路径
///[参数2] 存储包的github路径
///[参数3] 需要修改的工程路径
/// 
pub fn install(pkgs_path : &String,path : &String,vcproj_path : &String) -> Result<()>{
    //println!("[T]install");

    //校验路径是否合法(后缀必须是.git)
    let git_path = &path::PathBuf::from_str(path).unwrap();

    if git_path.extension().unwrap() != "git" {
        panic!("[E]error path suffix : {:?}",git_path.extension().unwrap());
    }

    //利用git从github仓库下载包(事先需要安装git)



    
    //创建pkgs目录(已存在的话不覆盖)
    //Command::new("cmd").arg("/C").arg(format!("mkdir {}",pkgs_path)).output().expect("[E]cmd excute error");

    //cd到新创建的pkgs目录
    Command::new("cmd").arg("/C").arg(format!("cd \\{} {} ",&pkgs_path[0..1],pkgs_path)).output().expect("[E]cmd excute error");
    
    //查看当前cmd所在路径,判断cd命令是否成功
    //cmd::dir();

    //开始clone

    println!("[I]start clone {}...",path);
    println!("[I]please wait for a while");
    Command::new("cmd").arg("/C").arg("git clone").arg(path).output().expect("[E]git clone error");


    //判断是否clone成功
    //git clone因为网络失败是常事

    //获得包名
    let pos_dot = path.rfind('.').expect("[E]path dont have '.git'");
    let pos_slash = path.rfind('/').expect("[E]path dont have '/'");
    let pkg_name = &path[pos_slash+1..pos_dot];

    println!("[I]package name : {}",pkg_name);

    //
    let mut t = pkgs_path.clone();
    t.push('\\');
    t.push_str(&pkg_name);
    let t = path::PathBuf::from_str(t.as_str()).unwrap();

    println!("[I]pkgs_path : {}",t.display());
    if !t.exists(){
        println!("[E]clone failed : Time Out or Git path is invalid");
        return Ok(());
    }
    println!("[I]clone done");

    fix::fix_vcxproj(&t.into_os_string().into_string().unwrap(),vcproj_path);

    Ok(())
}
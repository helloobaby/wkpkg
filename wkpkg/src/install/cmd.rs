//调用cmd命令

#[warn(dead_code)]  
use std::process::Command;

pub fn utf_8(){
    Command::new("cmd").arg("/C").arg("chcp 65001");
}

pub fn gbk(){
    Command::new("cmd").arg("/C").arg("chcp 936");
}

pub fn dir(){
    
    utf_8();

    let output = Command::new("cmd").arg("/C").arg("dir").output().expect("[E]cmd excute error");
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}
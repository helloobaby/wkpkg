use std::string;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}



pub fn fix_vcxproj(pkg_path : &String,vcproj_path : &String){
    println!("[vcxproj at] {}",vcproj_path);

    //把所有文件夹加入包含目录,把.c/.cpp文件加入项目

    let walker = WalkDir::new(pkg_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        println!("{}", entry.unwrap().path().display());
    }

    
    

}
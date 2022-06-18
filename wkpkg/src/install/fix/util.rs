use std::string::String;
use std::path;
use std::str::FromStr;
use std::ops::Deref;
use std::io::Result;


pub fn get_name_from_path(path : &String) -> String{

    let pos_dot = path.rfind('.');
    let pos_slash = path.rfind('\\').expect("[E]path dont have '/'");
    let pkg_name;
    if pos_dot.is_none(){
        pkg_name = &path[pos_slash+1..];}
    else{
        pkg_name = &path[pos_slash+1..pos_dot.unwrap()];
    }

    return String::from(pkg_name);

}
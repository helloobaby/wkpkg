///
/// 
///
/// 
use std::vec;
use std::string;
use std::io::Result;

fn main() -> Result<()>{
    let args : Vec<String> = std::env::args().collect();
    let current_dir = std::env::current_dir()?;
    
    //println!("[I]root path {}",args[0]);
    //println!("[I]current dir{}",current_dir.display());



    Ok(())
}

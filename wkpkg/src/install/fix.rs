use std::string;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};
use fast_xml::{Reader, Writer};
use fast_xml::events::{Event, BytesEnd, BytesStart,BytesText};
use std::io::Cursor;
use std::fs;

mod util;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}



pub fn fix_vcxproj(pkg_path : &String,vcproj_path : &String){
    println!("[vcxproj at] {}",vcproj_path);

    let mut _cpp : Vec<String> = Vec::new();
    let mut _hpp : Vec<String> = Vec::new();

    let walker = WalkDir::new(pkg_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {

        println!("{}", entry.as_ref().unwrap().path().display());

        //entry合法,并且文件有后缀
        if(!entry.is_err() && !entry.as_ref().unwrap().path().extension().is_none()){

        if(entry.as_ref().unwrap().path().extension().expect("") == "h"
    ||entry.as_ref().unwrap().path().extension().unwrap() == "hpp" ||
    entry.as_ref().unwrap().path().extension().unwrap() == "hxx" ||
    entry.as_ref().unwrap().path().extension().unwrap() == "hh")
        {
                _hpp.push(String::from(entry.as_ref().unwrap().path().to_str().unwrap()));
        }

        if(entry.as_ref().expect("[E]invalid entry").path().extension().expect("") == "c"
        ||entry.as_ref().unwrap().path().extension().unwrap() == "cc" ||
        entry.as_ref().unwrap().path().extension().unwrap() == "cpp" ||
        entry.as_ref().unwrap().path().extension().unwrap() == "cxx")
            {
                    _cpp.push(String::from(entry.unwrap().path().to_str().unwrap()));
            }
    }
}

    //打印出收集的文件名
    
    println!("[I]header files :",);
    for file in _hpp.iter().clone(){
        println!("[I]{}",file);
    }
    
    println!("[I]source files :",);
    for file in _cpp.iter().clone(){
        println!("[I]{}",file);
    }


    //首先需要在.vcxproj.filters文件中加个filter,filter名字为包名
    let pkg_name = util::get_name_from_path(pkg_path);
    println!("[I]pkg_name : {}",pkg_name);

    let mut buf = Vec::new();
    let mut reader = Reader::from_file(vcproj_path).expect("[E]error read visual studio project file");
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    
    //因为有很多出ItemGroup,我们只需要改一处
    let mut is_already_fix = false;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                writer.write_event(Event::Start(e.clone()));

                let str = String::from_utf8_lossy(e.name());

                let mut elem = BytesStart::owned(b"Filter".to_vec(), "Filter".len());
                elem.push_attribute(("Include", pkg_name.as_str()));
                
                //println!("[xml]{}",str);

                //找到ItemGroup标签
                if(str == "ItemGroup" && !is_already_fix){
                    writer.write_event(Event::Text(BytesText::from_escaped_str("\r\n")));

                    is_already_fix = true;
                    assert!(writer.write_event(Event::Start(elem)).is_ok());

                    writer.write_event(Event::Text(BytesText::from_escaped_str("\r\n")));
                    //中间补一个UniqueIdentifier
                    let id = BytesText::from_escaped_str
                    ("<UniqueIdentifier>{F6FC5500-8EC3-AA91-C307-365E97A691BB}</UniqueIdentifier>");
                    writer.write_event(Event::Text(id));
                    
                    
                    //还得补个end上去
                    writer.write_event(Event::Text(BytesText::from_escaped_str("\r\n")));
                    let end = BytesEnd::owned(b"Filter".to_vec());
                    writer.write_event(Event::End(end));

                    writer.write_event(Event::Text(BytesText::from_escaped_str("\r\n")));

                    //然后把相应的文件加进去
                    let mut hfmt;
                    let mut cfmt;

                    for file in _hpp.iter().clone(){
                        hfmt = format!("<ClInclude Include=\"{}\">\r\n<Filter>{}</Filter>\r\n</ClInclude>\r\n",
                    file,pkg_name);
                    let t = BytesText::from_escaped_str(hfmt);
                    writer.write_event(Event::Text(t));
                    }

                    for file in _cpp.iter().clone(){
                        cfmt = format!("<ClCompile Include=\"{}\">\r\n<Filter>{}</Filter>\r\n</ClCompile>\r\n",
                    file,pkg_name);
                    let t = BytesText::from_escaped_str(cfmt);
                    writer.write_event(Event::Text(t));
                    }


                    

                }


               
        },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(e) => assert!(writer.write_event(&e).is_ok()), //不关心的其他字段原模原样写进去
            _ => (),
        }
        buf.clear();
    }    
    let result = writer.into_inner().into_inner();
    //println!("{:?}",result);

    let ioret = fs::write(vcproj_path,result);
    if(ioret.unwrap() != (())){
        println!("[E]fix vcxproj file failed!");}

}
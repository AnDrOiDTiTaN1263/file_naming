use std::{fs::{metadata,  File, rename},thread::sleep, fmt::Debug, };
use std::io;
use rexif;


use std::time::Duration as dur;
use std::path::Path;

fn change_name(path: String, recurse: bool)->std::io::Result<()>{
    //original file path
    let mut opath = String::from(&path);
    let ppath = Path::new(&path);
    let md = metadata(&path)?;

    if  md.is_dir() && recurse{
        println!("found directory: starting recursion for all files and subfolders");
        for entry in ppath.read_dir().expect("read_dir() call failed") {
            if let Ok(entry) = entry {
                match entry.path().to_str() {
                    Some(s)=>{
                        change_name(s.to_string(), recurse).expect("something went wrong for file: {s:?}");
                    }
                    None=>{
                        println!("something went wrong getting the file name");
                    }
                }
            }
        }
    }else if md.is_file(){
        match rexif::parse_file(&opath) {
            Ok(exif) => {  
                println!("renaming file: {:?}",opath);
                let mut date_time_idx:usize = 0;
                let mut entries = exif.entries;
                for entry in &entries{
                    if entry.tag.to_string().contains("Date of original image") {
                        break;
                    }else {
                        date_time_idx +=1;
                    }
                }
                let mut date = String::from(&entries[date_time_idx].value_more_readable.to_string());
                date = date.replace(":", "-");
                date = date.replace(" ", "-");
                let mut new_path = String::from(&opath);
                let mut file_index = 0;
                match opath.rfind("/"){
                    Some(x)=>{
                        file_index = x;
                        new_path.truncate(file_index);

                    },None=>{
                        println!("could not find the last / in the files path");
                    }
                }
                new_path +="/";
                new_path += &date;
                new_path +=".jpeg";
                rename(opath, new_path).expect("something went wrong when renaming the file!");
            }
            Err(e) => {
                println!("Error in {}", &opath);
                println!("{e:?}");
            }
        }
    }
    else if md.is_dir() && !recurse{
        println!("You have not given program permission to recurse into sub-folders, but given path is a directory, please give either recursion perms or give a file path not directory path")
    }
    Ok(())
}

fn take_name_input()->String{
    let mut ok:bool = false;
    let mut input:String = String::new();
    while !ok{
        println!("Please provide path to the files: ");
        let stdin = io::stdin();
        stdin.read_line( &mut input).expect("something went wrong while reading input");

        //remove \n character at end
        input.remove(input.len()-1);
        println!("you entered: {:?}", input);
        println!("Press Y to confirm or N to enter again");
        let mut confirm_input = String::new();
        stdin.read_line(&mut confirm_input).expect("something went wrong with the confirmation input");
        confirm_input.remove(confirm_input.len()-1);
        if confirm_input == "Y" || confirm_input == "y" || confirm_input == "Yes"|| confirm_input == "yes"{
            ok = true;
        }else{
            input.clear();
        }
    }


    input
}


fn main() { 
    println!("\ntime will be in UTC time not AEST\n");
    change_name(take_name_input(), true).expect("something went wrong in main fn call");
    
}

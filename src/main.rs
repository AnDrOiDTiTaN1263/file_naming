use std::{fs::{metadata, rename}};
use std::io;
use rexif;
use std::path::Path;

fn change_name(path: String, recurse: bool)->std::io::Result<()>{
    //original file path
    let opath = String::from(&path);
    let ppath = Path::new(&path);
    let md = metadata(&path)?;

    if  md.is_dir() && recurse{
        println!("found directory: starting recursion for all files and subfolders");
        for entry in ppath.read_dir().expect("read_dir() call failed") {
            if let Ok(entry) = entry {
                match entry.path().to_str() {
                    Some(s)=>{
                        //call function again on the file path 
                        //will recurse if the new file path provided is also a folder
                        change_name(s.to_string(), recurse).expect("something went wrong for file: {s:?}");
                    }
                    None=>{
                        println!("something went wrong getting the file path for the file: {:?}",entry.path());
                    }
                }
            }else{
                println!("was not a valid file!");
            }
        }
    }else if md.is_file(){
        match rexif::parse_file(&opath) {
            Ok(exif) => {  
                println!("renaming file: {:?}",opath);
                let mut date_time_idx:usize = 0;
                let entries = exif.entries;
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
                match opath.rfind("/"){
                    Some(x)=>{
                        new_path.truncate(x);

                    },None=>{
                        println!("could not find the last / in the files path");
                    }
                }
                new_path +="/";
                new_path += &date;
                new_path +=".jpg";
                rename(opath, new_path).expect("something went wrong when renaming the file!");
            }
            Err(e) => {
                println!("Error in {}", &opath);
                println!("{e:?}");
            }
        }
    }
    else if md.is_dir() && !recurse{
        println!("You have not given program permission to recurse into sub-folders, but given a path which is a directory, please give either recursion permission or give a file path this is not directory path")
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
        println!("you entered: {:?}\n", input);
        println!("Press Y to confirm or N to enter again");
        let mut confirm_input = String::new();
        stdin.read_line(&mut confirm_input).expect("something went wrong with the confirmation input");
        confirm_input.remove(confirm_input.len()-1);
        if confirm_input.contains("y"){
            ok = true;
        }else{
            input.clear();
        }
    }


    input
}

fn take_recurse_input()->bool{
    let mut recurse = false;
    let mut input:String = String::new();
    println!("Would you like the program to recurse into directories and sub-directories? y/n");
    let stdin = io::stdin();
    stdin.read_line( &mut input).expect("something went wrong while reading input");
    //remove \n character at end
    input.remove(input.len()-1);
    if input.contains("y") {
        recurse = true;
    }
    recurse
}

fn main() { 
    println!("\ntime will be in UTC time not AEST\n");
    match change_name(take_name_input(), take_recurse_input()) {
        Ok(_)=>{
            println!("Finished changing names to required format");
        }Err(e)=>{
            println!("change name function returned an error:\n{:?}",e);
        }
    }
    
}

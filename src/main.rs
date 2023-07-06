
use std::{fs::{File,rename}};
use std::io::BufReader;
use std::io;
use chrono::Utc;

use exif::{Reader, Tag, In};
use std::path::Path;

fn calc_date(dur:std::time::Duration)->String{
    let mut today = Utc::now();
    today = today.checked_sub_signed(chrono::Duration::seconds(dur.as_secs() as i64)).unwrap();
    let mut date = today.to_string();
    date.truncate(date.len()-11);
    date.replace("T", "-").replace(":", "-").replace(" ", "-")
}

fn make_file_path(date:String, original_path:String)->Option<String>{
    let mut new_path:String = String::from(original_path);
    if let Some(sidx) = new_path.rfind("/"){
        new_path.truncate(sidx);
        new_path += "/";
        new_path += &date;
        println!("{new_path:?}");
    }else{  
        println!("Please enter a proper file path, not just a file name");
        return None
    }
    Some(new_path + ".jpg")
}

fn change_name(path:String, recurse: bool){
    //using kamadak-exif
    //opath not to be changed
    let opath:String = String::from(&path); 
    let ppath:&Path = Path::new(&path);
    match File::open(opath){
            Ok(file)=>{
                match file.metadata(){
                    Ok(md)=>{
                        if md.is_dir() && recurse{
                            println!("given directory path, recursing!");
                            for entry in ppath.read_dir().expect("something went wrong reading the directory"){
                                if let Ok(entry) = entry{
                                    match entry.path().to_str(){
                                        Some(s)=>{
                                            change_name(s.to_string(), recurse);
                                        }
                                        None=>{
                                            println!("could not successfully turn path into a string: {:?}", entry.path());
                                        }
                                    }
                                }else if let Err(e) = entry{
                                    println!("encountered the following error: {e:?}");
                                }
                            }
                        }else if md.is_file(){
                            match Reader::new().read_from_container(
                                &mut BufReader::new(&file)){
                                    Ok(exif)=>{
                                        match exif.get_field(Tag::DateTimeDigitized, In::PRIMARY){
                                            Some(date)=>{
                                                match  make_file_path(
                                                    date.display_value().to_string()
                                                        .replace(" ", "-").replace(":", "-")
                                                        ,String::from(&path)) {
                                                    Some(new_path)=>{
                                                        //this is where we rename the file
                                                        match  rename(String::from(&path), new_path){
                                                            Ok(_)=>{
                                                                println!("Success...");
                                                            }Err(e)=>{
                                                                println!("something went wrong at the last step when actually renaming the file");
                                                            }
                                                        }
                                                    }
                                                    None=>{
                                                        //do not rename something went wrong when making the new path
                                                        println!("You gave a bad path, the new path returned was none --- aborting for this file");
                                                    }

                                                } 
                                            
                                            }None=>{
                                                match md.created() {
                                                        Ok(time)=>{
                                                            println!("couldn't find the date field in the exif data, resorting to file metadata to get creation time");
                                                            match make_file_path(calc_date(time.elapsed().unwrap()), String::from(&path)){
                                                                Some(new_path)=>{
                                                                    match rename(String::from(&path), new_path){
                                                                    Ok(_)=>{    
                                                                        println!("success")
                                                                    }
                                                                    Err(e)=>{
                                                                        println!("something went wrong at the last step {e:?}");
                                                                    }
                                                                }
                                                                }
                                                                None=>{
                                                                    //do not rename something went wrong when making the new path
                                                                    println!("You gave a bad path, the new path returned was none --- aborting for this file");
                                                                }
                                                            }
                                                            
                                                        }Err(e)=>{
                                                            println!("something went wrong resulting in error: {e:?}")
                                                        }
                                                }
                                            }
                                        }
                                    }Err(e)=>{
                                        println!("encountered an error initialising exif: {e:?}");
                                        println!("a likely cause is that a PNG image was supplied, resorting to file creation date");
                                           match make_file_path(
                                                calc_date(
                                                    file.metadata().unwrap().created()
                                                    .unwrap().elapsed().unwrap()),
                                                String::from(&path)){
                                                    Some(new_path)=>{
                                                        match rename(String::from(&path), new_path){
                                                            Ok(_)=>{
                                                                println!("success...");
                                                            }Err(e)=>{
                                                                println!("Something went wrong at the last step, renaming: {e:?}");
                                                            }
                                                        }
                                                    }None=>{
                                                        println!("You gave a bad path, the new path returned was none --- aborting for this file");
                                                    }
                                                }
                                    }
                                }
                        }
                        else if md.is_dir() && !recurse{
                            println!("You did not give recursion permission but supplied a directory");
                        }
                        else{
                            println!("Could not resolve whether a file or a directory was given");
                        }
                    }Err(e)=>{
                        println!("Encountered an error while reading the metadata of the file/directory: {e:?}");
                    }
                }
            }Err(e)=>{
                    println!("There was an error opening the file/directory!");
                    println!("error was: {e:?}");
                }
            }
    
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
        if confirm_input.starts_with("y") || confirm_input.starts_with("Y"){
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
    println!("Using the kamadak-exif rust crate: https://crates.io/crates/kamadak-exif");
    println!("\ntime will be in UTC time not AEST\n");
    change_name(take_name_input(), take_recurse_input());
    println!("\nfinished");
}

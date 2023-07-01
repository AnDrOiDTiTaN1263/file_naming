use std::{fs::{metadata, rename, File},thread::sleep};
use std::io;
use chrono::{prelude::{DateTime, Utc},  Duration};

use std::time::Duration as dur;
use std::path::Path;
fn iso8601(st: &std::time::SystemTime) -> String {
    let mut dt: DateTime<Utc> = st.clone().into();
    dt = dt + Duration::hours(10);
    format!("{}", dt.format("%+"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

fn format_iso_time(mut s: String)->String{
    s = s.replace("T", " ");
    s.truncate(19);
    s
}

fn change_name(path: String, recurse: bool)->std::io::Result<()>{
    let opath = String::from(&path);
    let mut mpath = String::from(&path);
    let ppath = Path::new(&path);
    let md = metadata(&path)?;

    if  md.is_dir() && recurse{
        println!("found directory: starting recursion for all files and subfolders");
        for entry in ppath.read_dir().expect("read_dir call failed") {
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
        if let Ok(time) = md.created(){
            let mut creation_time:String = iso8601(&time);
            creation_time = format_iso_time(creation_time);
            let file_extension = mpath.rfind(".");
            println!("going from: {mpath:?}");
            match file_extension {
                Some(s)=>{
                    let fe = mpath.split_off(s);
                    let file_name_idx = mpath.rfind("/");
                    match file_name_idx {
                        Some(idx)=>{
                            mpath.truncate(idx);
                            mpath += "/";
                            mpath +=  &creation_time;
                            mpath += &fe;
                            
                            rename(opath, mpath).expect("something went wrong when renaming");

                        }None=>{
                            println!("file name index not found ");
                        }
                    }
                }None=>{
                    println!("Couldn't find the file extension!");
                }
            }
            
            

        // rename(path, )

        } else {
            println!("Not supported on this platform or filesystem");
        }
    }
    else if md.is_dir() && !recurse{
        println!("You have not given program permission to recurse into sub-folders, but given path is a directory, please give either recursion perms or give a file path not directory path")
    }
    Ok(())
}
#[allow(unused)]
fn create_test_files(number_of_files: i8){
    for x in 0..number_of_files{
        let mut path:String = String::from("src/test_files/");
        path += &x.to_string();
        path += ".test";
        sleep(dur::from_millis(250));
        File::create(path).expect("something went wrong");
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
    // create_test_files(3);
    let mut recurse: bool = false;
    let mut input:String = String::new();
    println!("Would you like the function to recurse into sub folders?: Y/N ");
    let stdin = io::stdin();
    stdin.read_line( &mut input).expect("something went wrong while reading input");
    input.remove(input.len()-1);
    if input == "Y" || input == "y" || input == "Yes" || input == "yes"{
        recurse = true;
    }


    change_name(take_name_input(), recurse).expect("something went wrong in main function call!");
    
}

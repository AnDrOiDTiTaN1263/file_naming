use std::collections::HashMap;
// use std::env;
use std::fs::{File, create_dir, read_dir, metadata, copy};
use std::io::{BufReader, self};
use chrono::Utc;
use exif::{Reader, Tag, In};
use std::env;

const VALID_FILE_EXTENSIONS:[&str;2] = ["png", "jpg"];
const OS:&str = env::consts::OS;  

fn calc_date(dur:std::time::Duration)->String{
    let mut today = Utc::now();
    today = today.checked_sub_signed(chrono::Duration::seconds(dur.as_secs() as i64)).unwrap();
    let mut date = today.to_string();
    date.truncate(date.len()-11);
    date.replace("T", "-").replace(":", "-").replace(" ", "-").replace(".", "-")
}

fn create_dir_structure(path_map: &HashMap<String, Vec<String>>, dest_file_path:String){
    if let Err(_) = read_dir(dest_file_path.clone()){
        //if the destination directory does not exist
        println!("destination directory does not exist, trying to create it...");
        create_dir(dest_file_path.clone()).unwrap();
        create_dir_structure(path_map, dest_file_path.clone());

    }
    for entry in path_map{
        let mut n_date = String::from(entry.0);
        n_date.truncate(7);
        let month = n_date.split_off(5);
        n_date.truncate(4);
        match read_dir(String::from(&dest_file_path.clone())+"/"+&n_date){
            Ok(_)=>{
                //year exists, check month
                match read_dir(String::from(&dest_file_path)+"/"+&n_date+"/"+&month){
                    Ok(_)=>{
                        // year/month dir exists:
                    }Err(_)=>{
                        //month dir does not exist
                        create_dir(String::from(&dest_file_path)+"/"+&n_date+"/"+&month).expect("something went wrong making the new dir");
                    }
                }
            }Err(_)=>{
                //year dir does not exist
                match read_dir(String::from(&dest_file_path)+"/"+&n_date+"/"+&month){
                    Ok(_)=>{
                        //year/month dir exists -> should never be the case
                    }Err(_)=>{
                        //year dir didn't exist, so we creat both the year dir, and the associated month dir
                        create_dir(String::from(&dest_file_path)+"/"+&n_date).expect("something went wrong creating the year directory");
                        create_dir(String::from(&dest_file_path)+"/"+&n_date+"/"+&month).expect("something went wrong making the new dir");
                    }
                }
            }
        }
    }   
}

fn append_to_vec(hm:& mut HashMap<String, Vec<String>>,key:String, path:String){
    //try to append the path of the new file to the vector of existing paths (if they exist)
    //else create a hashmap key value pair and create new vector with the element path
    match hm.get_mut(&key){
        Some(value)=>{
            value.push(path);
        }None=>{
            hm.insert(key, vec![path]);
        }
    }
}

fn resolve_paths(path:String)->HashMap<String, Vec<String>>{
    //only run when given a folder path
    let mut ret:HashMap<String, Vec<String>> = HashMap::new();
    let md = metadata(String::from(&path)).unwrap();
    if md.is_dir(){
        //sort files here
        let dir = read_dir(String::from(&path)).unwrap();
        
        for dir_res in dir{
            let dir_res= dir_res.unwrap();
            let dir_path = dir_res.path();
            let path = dir_path.to_str().expect(&format!("cannot convert path of {:?} ",dir_path.to_string_lossy()));
            let md = dir_res.metadata().unwrap();
            // println!("{:?}",&path.get(path.rfind(".").unwrap()+1..).unwrap());
            if md.is_dir(){
                //recurse 
                ret.extend(resolve_paths(String::from(dir_path.to_str().unwrap())));

            }else if VALID_FILE_EXTENSIONS.contains(&path.get(path.rfind(".").unwrap()+1..).unwrap_or_default()){

                let file_date = get_file_date(path.to_string().clone());
                append_to_vec(&mut ret, file_date, path.to_string().clone());
            }
        }   
    }else{
        //if there's only 1 file we don't really want to check anything and just copy it over
        
        append_to_vec(&mut ret, get_file_date(String::from(&path)), path.clone());
    }
    return ret;
}

fn get_file_date(path:String)->String{
    //returns empty string if not good
    let mut ret:String = String::new();
    match File::open(String::from(path.clone())){
        Ok(file)=>{
            match Reader::new().read_from_container(&mut BufReader::new(&file)){
                Ok(exif)=>{
                    println!("exif data found for: {:?} ", path.clone());
                    match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY){
                        Some(date)=>{
                            // println!("EXIFdate: {:?}",date.display_value().to_string());
                            ret = date.display_value().to_string();
                            ret = ret.replace(":", "-").replace(" ", "-");
                        }
                        None=>{
                            ret = calc_date(file.metadata().unwrap().created().unwrap().elapsed().unwrap());
                            ret = ret.replace(":", "-").replace(" ", "-");
                        }
                    }
                }Err(_)=>{
                    println!("No Exif Data found for file: {:?}", path);
                    ret = calc_date(file.metadata().unwrap().created().unwrap().elapsed().unwrap());
                    ret = ret.replace(":", "-").replace(" ", "-");
                }
            }
        }
        Err(e)=>{
            println!("encountered an error: {e:?}");
        }
    }
    ret
}

fn resolve_dest_path(path:String, dest_file_path:String, directory_separator:&str)->String{
    let mut ret = String::from(dest_file_path.clone());
    println!("{path:?}");
    ret+=directory_separator;
    ret += &path[0..4];
    ret +=directory_separator;
    ret += &path[5..7];
    ret +=directory_separator;
    ret += &path;
    println!("{ret:?}.jpg");
    ret
}

fn copy_files(path_map: &HashMap<String, Vec<String>>, dest_file_path:String, directory_separator:&str){
    for (k,v) in path_map{
        let file_extension = v[0].get(v[0].rfind(".").unwrap()..).unwrap();
        println!("file extension: {file_extension:?}");
        println!("v[0]={:?}; destination path resolved as: {:?}",v[0].clone(),resolve_dest_path(k.clone(), dest_file_path.clone(), directory_separator)+v[0].get(v[0].rfind(".").unwrap()..).unwrap());
        //only do the first file
        match copy(v[0].clone(),resolve_dest_path(k.clone(), dest_file_path.clone(),directory_separator)+v[0].get(v[0].rfind(".").unwrap()..).unwrap()){
            Ok(_)=>{

            }Err(e)=>{
                println!("encountered error whilst copying: {e:?}");
            }
        }
        if v.len() >1 {
            for counter in 1..v.len(){
                //copy the files over given the counter i.e. path_to_dest_folder/yyyy/mm/yyyy-mm-dd-hh-mm-ss-c.jpg
                match  copy(v[0].clone(), resolve_dest_path(k.clone(), dest_file_path.clone(), directory_separator)+"-"+&counter.to_string()+v[0].get(v[0].rfind(".").unwrap()..).unwrap()){
                    Ok(_)=>{

                    }Err(e)=>{
                        println!("encountered an error: {e:?}");
                    }
                }   
            }   
        }
    }
}

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
fn print_help(){
    println!("the program expects 2 command line arguments, they are:");
    println!("[path_to_files] and the keyword \"recurse\" which will allow the program to recurse into sub folders given a directory");
    println!("An example is: cargo run /test/test1 recurse");
    println!("Another example is: cargo run -- /test/test1");

}


fn main() { 
    // grab os info
      
    // linux / unix based systems
    let mut directory_separator:&str = "/";
    if OS == "windows"{
        directory_separator = "\\";
    }


    println!("Using the kamadak-exif rust crate: https://crates.io/crates/kamadak-exif");
    println!("\ntime will be in UTC time not AEST\n");
    println!("\n\n");
    println!("detected OS: {OS:?}");
    println!("\n\n");
    // print_help();
    let args:Vec<String> = env::args().collect();
    println!("args: {args:?}");
    if args.len() == 1 {
        println!("### WARNING ###\n");
        print_help();
    }else{
        let orig_file_path:String = args[1].clone();
        println!("got: {}",orig_file_path.clone());
        let dest_file_path:String = args[2].clone();
        println!("will copy over to: {}",dest_file_path.clone());

        println!("starting...");

        let path_map = resolve_paths(orig_file_path.clone());
        println!("finished getting new filepaths...\nstarting to create directories needed");
        create_dir_structure(&path_map, dest_file_path.clone());
        println!("finished creating the directories needed... \nstarting the copying process...");
        copy_files(&path_map, dest_file_path.clone(), directory_separator);

        println!("finished");
    }
}

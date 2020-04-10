extern crate trash;
extern crate walkdir;
use std::{env};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().expect("Can't get current dir!");
    let desired_dir = match args.len() > 1 {
        true => &args[1],
        false => cwd.to_str().unwrap()
    };
    
    for entry in WalkDir::new(desired_dir) {
        let entry = entry.expect("Can't get entry!");
        let path = entry.path();
        if !path.is_dir() {
            let filename = entry.file_name();
            let filenamestr = filename.to_str().expect("Can't get filename!");
            let collector : Vec<&str> = filenamestr.matches(r"^.* 2\..*$").collect();
            if filenamestr.ends_with(" 2") || filenamestr.ends_with(".icloud") || collector.len() > 0 {
                let rslt = trash::remove(path.canonicalize().expect("Can't get canonical path!").to_str().expect("Can't get canonical path!"));
                match rslt {
                    Ok(_x) => println!("Trashed {}", filenamestr),
                    Err(_x) => println!("Couldn't trash {}", filenamestr)
                };
            }
        }
    }
}
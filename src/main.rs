extern crate trash;
extern crate walkdir;
extern crate executors;
extern crate regex;
use std::{env};
use walkdir::WalkDir;
use executors::*;
use executors::threadpool_executor::ThreadPoolExecutor;
use std::sync::mpsc::channel;
use std::sync::Arc;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().expect("Can't get current dir!");
    let desired_dir = match args.len() > 1 {
        true => &args[1],
        false => cwd.to_str().unwrap()
    };

    let n_workers = 64;
    let pool = ThreadPoolExecutor::new(n_workers);
    let (tx, rx) = channel();
    let re = Regex::new(r"^.* [1-9][0-9]*(?:\..*)?$").expect("Invalid regex syntax");
    for dirent in WalkDir::new(desired_dir) {
        match dirent {
            Ok(goodent) => {
                let eclone = Arc::new(goodent.clone());
                let path = Arc::new(eclone.path());
                if !path.is_dir() {
                    let filename = Arc::new(eclone.file_name().clone());
                    let filenamestr = filename.to_str().expect("Can't get filename!").clone();
                    let canon = path.canonicalize().unwrap();
                    let canonstr = Arc::new(canon.clone());
                    if !filenamestr.ends_with(".pptx") && (filenamestr.ends_with(".icloud") || re.is_match(filenamestr)) {
                        let txa = tx.clone();
                        pool.execute(move|| {
                            let rslt = trash::remove(canonstr.to_str().unwrap());
                            match rslt {
                                Ok(_x) => txa.send(Arc::new(format!("Trashed {}", canonstr.to_str().unwrap()))).expect("channel will be there waiting for the pool"),
                                Err(_x) => txa.send(Arc::new(format!("Couldn't trash {}", canonstr.to_str().unwrap()))).expect("channel will be there waiting for the pool")
                            };
                        });
                    }
                }
            },
            Err(_) => ()
        }
    }
    pool.shutdown().expect("pool won't shut down");
    drop(tx);
    let mut gogo = true;
    while gogo {
        let recv : Result<Arc<String>, std::sync::mpsc::RecvError> = rx.recv();
        match recv {
            Ok(x) => println!("{}", x),
            Err(_) => gogo = false
        }
    }
}
extern crate zip;

use std::{io, fs};
use std::io::Read;

use zip::ZipArchive;
use zip::result::ZipError;

fn main() {
    let fname = std::path::Path::new("/sample.zip");
    let file = fs::File::open(fname).expect("can't open file");

    let mut zip = ZipArchive::new(file).expect("can't open file as archive");

    for i in 0..zip.len() {
        let file = zip.by_index(i).unwrap();

        println!("{}. {}", i, file.name());
    }

    for i in 1.. {
        let name = format!("x_{}.json", i);
        println!("reading {}...", &name);
        
        match zip.by_name(&name) {
            Ok(mut file) => {
                let mut content = String::with_capacity(file.size() as usize);
                match file.read_to_string(&mut content) {   
                    Ok(_size) => {
                        println!("{}", content);
                    }
                    Err(error) => {
                        let message = format!("{:?}", error);
                        println!("can't read as text: {}", message);
                    }
                }
            }
            Err(ZipError::FileNotFound) => {
                println!("file not found!");
                break;
            }
            Err(error) => {
                let message = format!("{:?}", error);
                panic!("{}", message);
            }
            
        }

    }
}

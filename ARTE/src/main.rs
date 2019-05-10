use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error; 
use std::io::{self, Write};

fn NewFile(mut filename: String) -> io::Result<()>{ //function to create file
    println!("Creating {}", filename);
    let path = Path::new(&filename);
    let mut file = File::create(&path);
    println!("file {} created", filename);
    return file;
}

fn main() {
    let args: Vec<String> = env::args().collect(); //accepts cli arguements 
    let filename =&args[1]; //takes file name from arg
    let path = Path::new(filename); //saves path name
    let display = path.display();

    println!("opening file: {}", filename); 
    let mut file = match File::open(&path){ //tries to open the file and set variable file to it
    
        //Err(why)=>panic!("Couldn't open {}: {}", display, why.description()),
        Err(file) => { 
            //let newFilePath = Path::new(&file);
            //let newFileDisplay = newFilePath.display();
            file = NewFile(filename.to_string()); //if it fails create file
    },
    };
    
    let mut s = String::new(); 
    match file.read_to_string(&mut s){ //s is now the contents of the file
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains: \n{}", display, s),
    }
    println!("File = {}", file);
}

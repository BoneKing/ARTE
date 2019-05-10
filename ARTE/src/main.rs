//File: main.rs
//Created by Andy Mahoney
//on: 5/9/19
//last updated: 5/10/19
//contact: andym AT fortrash DOT com

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error; 
//use std::io::self;
//use std::io::Read;
//use std::io;
use std::fs::OpenOptions;
use std::fs;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect(); //accepts cli arguements 
    let filename =&args[1]; //takes file name from arg
    let path = Path::new(filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
    println!("opening file: {}", filename); 
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    let mut contents: String = fs::read_to_string(path).expect("bad read"); //contents now equals the content of the file
    println!("File = {}", filename);
    println!("Contents = {}", contents);
}

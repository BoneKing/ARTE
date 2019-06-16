//file created by Andy Mahoney
//last edited on 6/16/19

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
use std::io::{BufReader, BufWriter};
use ropey::Rope;

pub static displayHeight: i32 = 50;

fn main(){
    let args: Vec<String> = env::args().collect(); //accepts cli arguements 
    let filename =&args[1]; //takes file name from arg
    let path = Path::new(filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
}
fn mode_Select(){

}
fn open(&coordinates: (i32,i32,i32)){
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    let mut contents = Rope::from_reader(
        File::open(path).expect("bad path")
    ).expect("getting contents failed");
    let start_idx = contents.line_to_char(0);
    let end_idx = contents.line_to_char(displayHeight);
    println!("",contents.slice(start_idx..end_idx));
}
fn edit(&filename: String){

}

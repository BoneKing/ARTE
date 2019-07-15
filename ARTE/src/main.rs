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

pub static displayHeight: usize = 50;

fn main(){
    let args: Vec<String> = env::args().collect(); //accepts cli arguements 
    let filename =&args[1]; //takes file name from arg
    let path = Path::new(filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
    let mut totalSplit: i32 =1; //keep track of the number of splits that exist, used as arguements in functions
    let mut totalTab: i32 =1; //above but tabs
    let mut totalDesktop: i32 =1; // above but desktops
    let mut coordinates: (i32, i32, i32)=(1,1,1); //coordinates of split tab and desktop in that order to send as arguements 
    println!("Welcome to ARTE! \n E: Edit file \n O: open file \n S: new split \n T: new tab \n D: new Desktop \n W: Write to file \n Q: quit \n M: return to main menu "); //basic menu
    mode_Select();
}
fn mode_Select(){
    println!("This doesn't exist yet sorta");
    println!("Mode Select: \n E: Edit file \n O: open file \n S: new split \n T: new tab \n D: new Desktop \n W: Write to file \n Q: quit \n M: return to main menu "); //basic menu
    let mut menuOption = String::new(); 
    io::stdin().read_line(&mut menuOption).expect("input error on menuOption"); //gets menu option selected by user
    match menuOption.as_str(){
        "E" => edit(&filename),
        'O' => open(&coordinates)
        //_ => println!("Bad menu option"), //if none of the above is entered
    };
}
fn open(coordinates: &(i32,i32,i32),filename: &String){
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    //let path = Path::new(&file);
    let mut text = Rope::from_reader(
        BufReader::new(File::open(file))
    ).expect("bad file read into ropes");
    let start_idx = contents.line_to_char(0);
    let end_idx = contents.line_to_char(displayHeight);
    println!("",contents.slice(start_idx..end_idx));
}
fn edit(filename: &String){
    println!("Edit function under contruction");
}
//fn as_ref(&self) -> &T;
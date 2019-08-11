//file created by Andy Mahoney
//last edited on 7/24/19
extern crate ropey;
use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error; 
//use std::io::self;
use std::io::Read;
use std::fs::OpenOptions;
use std::fs;
use std::fs::read_to_string;
use std::io::{BufReader, BufWriter};
use ropey::Rope;

pub static displayHeight: usize = 50;

fn main(){
    let args: Vec<String> = env::args().collect(); //accepts cli arguements 
    let filename =&args[1]; //takes file name from arg
    let mut totalSplit: i32 =1; //keep track of the number of splits that exist, used as arguements in functions
    let mut totalTab: i32 =1; //above but tabs
    let mut totalDesktop: i32 =1; // above but desktops
    let mut coordinates: (i32, i32, i32)=(1,1,1); //coordinates of split tab and desktop in that order to send as arguements 
    /* working on if no file is inputed at start then to create a new one and go to menu.
    if(filename == ""){
        let args2: Vec<String> = "untitled.txt";
        let filename2 = &args2[1];
        mode_Select(&coordinates, &filename2);
    }
    */
    let path = Path::new(filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
    //println!("Welcome to ARTE! \n E: Edit file \n O: open file \n S: new split \n T: new tab \n D: new Desktop \n W: Write to file \n Q: quit \n M: return to main menu "); //basic menu
    println!("Welcome to ARTE! \n");
    mode_Select(&coordinates, &filename);
}
fn mode_Select(coordinates: &(i32,i32,i32),filename: &String){
    println!("This doesn't exist yet sorta");
    println!("Mode Select: \n E: Edit file \n O: open file \n S: new split \n T: new tab \n D: new Desktop \n W: Write to file \n Q: quit \n M: return to main menu "); //basic menu
    let mut menuOption = String::new(); 
    io::stdin().read_line(&mut menuOption).expect("input error on menuOption"); //gets menu option selected by user
    //println!("menuOption is {}",menuOption);
    if(menuOption.as_str().trim() == "O"){
        println!("going to  open function");
        open(&coordinates, &filename);
    }
    else if(menuOption.as_str().trim() == "E"){
        println!("going to  Edit function");
        edit(&filename);
    }
    else if (menuOption.as_str().trim() == "Q"){
        println!("Quitting Now");
    }
    else{
        println!("WARNING: Invalid Option \n");
        mode_Select(&coordinates, &filename);
    }
}
fn open(coordinates: &(i32,i32,i32),filename: &String){
    println!("you're inside the open option");
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    let fileContents=fs::read_to_string(filename).expect("bad read of file");
    //let path = Path::new(&file);
    /*let mut contents = Rope::from_reader(
        BufReader::new(File::open(file))
    ).expect("bad file read into ropes");
    */
    let mut contents = Rope::from_str(&fileContents);
    let mut start_idx = 0;
    start_idx = contents.line_to_char(start_idx);
    let mut end_idx = contents.line_to_char(start_idx+displayHeight);
    //println!("{}",contents.slice(start_idx..end_idx));
    printPage(start_idx, end_idx, contents);
}
fn edit(filename: &String){
    println!("Edit function under contruction");
}
fn printPage(mut start_idx: usize, mut end_idx: usize, contents: Rope){
    //println!("\n \n WE IN PRINT PAGE \n \n");
    println!("{}",contents.slice(start_idx..end_idx));
    //println!("\n \n WE IN PRINT PAGE \n \n");
    println!("start_idx = {}",start_idx);
    println!("end_idx = {}", end_idx);
    let mut action = String::new(); 
    io::stdin().read_line(&mut action).expect("input error on printPage action");
    if(action.trim() == "k"){
        println!("Action = {}", action);
        printPage(start_idx+1, end_idx+1, contents);
    }
    else if(action.trim() == "j"){
        println!("Action = {}", action);
        if(start_idx-1 < 0){ //THIS DOESN"T WORK YET
            printPage(start_idx,end_idx,contents);
        }
        else {
            printPage(start_idx-1, end_idx-1, contents);
        }
    }
    if(action == "q"){
    }
    /*
    else{
        printPage(start_idx, end_idx, contents);
    }
    */
}
//fn as_ref(&self) -> &T;
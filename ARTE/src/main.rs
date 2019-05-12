//File: main.rs
//Created by Andy Mahoney
//on: 5/9/19
//last updated: 5/12/19
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

static mut splitTotal: i32 = 0; //tells other functions how many splits currently exist
static mut tabTotal: i32 = 0; //tells other functions how many splits currently exist
static mut desktopTotal: i32 = 0; //tells other functions how many splits currently exist
static mut currentSplit: i32 = 0; //current split
static mut currentTab: i32 = 0; //curent tab
static mut currentDesktop: i32 = 0; // current desktop

struct split{ //a split is what we are calling a window with in a tab, there can be 1 split to tab or n splits in a tab
        content: String, //a vector of ropes where each rope is a lone from the file
        struct currentLine(String, i64), //what the current line is (content of line, line number) 
        //fileHistory, //stack of file history versions for the sessions
        filename: String, //filename of file open in split
        struct coordinates(i32,i32,i32), //(what split, what tab, what desktop)
        topLine: i64, //line number being displayed on top of split
        bottomLine: i64, //line number being displayed on bottom of split
}

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
    let lineDispMax: i64 = 50;
    let mut topLine: i64 =0;
    let mut bottomLine: i64 = topLine+lineDispMax;
    let screenheight: i64 = 50;
}

fn display(screenheight: i64){
    let mut topeLine: i64 =0;
    let mut bottomLine: i64 = topLine+screenheight;
    for i in screenheight {
        println!("{}    ", i+1);
        println!(content[i]);
        println!("\n");
    }
}

fn open(filename: String){
    let path = Path::new(filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    let mut contents: String = fs::read_to_string(path).expect("bad read"); //contents now equals the content of the file
    let split[splitTotal] = split{
        contents: fs::read_to_string(path).expect("bad read"), //contents now equals the content of the file
        currentLine: (content[0], 0),
        //fileHistory
        filename: filename,
        coordinates: (splitTotal, currentTab, CurrentDesktop),
        topLine: 0,
        bottomLine: topLine+screenheight,
    }

}
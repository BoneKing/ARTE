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
use std::io::{BufReader, BufWriter};
use ropey::Rope;

struct gen_info{
    splitTotal: usize; //tells other functions how many splits currently exist
    tabTotal: i32; //tells other functions how many splits currently exist
    desktopTotal: i32; //tells other functions how many splits currently exist
    currentSplit: i32; //current split
    currentTab: i32; //curent tab
    currentDesktop: i32; // current desktop
    screenheight: i64;
    topLine: i64; //top line displayed
    bottomLine: i64; //bottom line displayes
}

struct split{ //a split is what we are calling a window with in a tab, there can be 1 split to tab or n splits in a tab
        content: String, //a vector of ropes where each rope is a lone from the file
        currentLine: (String, i64), //what the current line is (content of line, line number) 
        //fileHistory, //stack of file history versions for the sessions
        filename: String, //filename of file open in split
        coordinates: (usize,i32,i32), //(what split, what tab, what desktop)
        topLine: i64, //line number being displayed on top of split
        bottomLine: i64, //line number being displayed on bottom of split
}

fn main() {
    let args: Vec<String> = env::args().collect(); //accepts cli arguements 
    let filename =&args[1]; //takes file name from arg
    let path = Path::new(filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
    let mut GenInfo = gen_info{ splitTotal: 0, tabTotal: 0, desktopTotal: 0, currentSplit: 0, currentTab: 0, currentDesktop: 0, screenheight: 50, topLine: 0, bottomLine: topLine + screenheight };
    println!("opening file: {}", filename); 
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    let mut contents = Rope::from_reader(
        File::open(path).expect("bad path")
    ).expect("getting contents failed"); 
    let start_idx = contents.line_to_char(0);
    let end_idx = contents.line_to_char(4);
    //.expect("bad read"); //contents now equals the content of the file
    println!("File = {}", filename);
    println!("Contents = {}", contents.slice(start_idx..end_idx));
    println!("contents line 4 {}", contents.line(3));
    let lineDispMax: i64 = 50;
    //let mut topLine: i64 =0;
    //let mut bottomLine: i64 = topLine+lineDispMax;
    //let screenheight: i64 = 50;
    //display(contents);
}
/* Under construction fines doubled
fn display(content){
    //let mut topLine: i64 =0;
    //let mut bottomLine: i64 = topLine+screenheight;
    let mut num = 0;
    while num != screenheight {
        println!("{}    ", num+1);
        println!("{}", content[num]);
        println!("\n");
        num-=-1; //lol 
    }
}
Leaving construction area */

unsafe fn open(filename: String){
    let path = Path::new(&filename); //saves path name
    let display = path.display(); //Honnestly I have no idea people just always have path followed by this so I figured it wouldn't hurt 
    let file = OpenOptions::new().read(true).write(true).create(true).open(filename); //sets read and write privalages to true, and creates file if its not present
    let mut contents: String = fs::read_to_string(path).expect("bad read"); //contents now equals the content of the file
    let mut content = Vec::new();
    let mut GenInfo2 = gen_info{ splitTotal: 0, tabTotal: 0, desktopTotal: 0, currentSplit: 0, currentTab: 0, currentDesktop: 0, screenheight: 50, topLine: 0, bottomLine: topLine + screenheight };
    let mut split = Vec::new();
    split[splitTotal] = split{
        content: fs::read_to_string(path).expect("bad read"), //contents now equals the content of the file
        currentLine: (content[0], 0),
        //fileHistory
        filename: filename,
        coordinates: (GenInfo2.splitTotal, GenInfo2.currentTab, GenInfo2.currentDesktop),
        topLine: 0,
        bottomLine: topLine+screenheight,
    }

}

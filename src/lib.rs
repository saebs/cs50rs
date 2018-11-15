mod pset1_rust;
mod pset2_crypto;

use pset1_rust::*; // aka C
use pset2_crypto::*; 
use std::env;
use std::process;
use std::io::Write;

pub fn run() {
    let cs50: Vec<String> = env::args().collect();
    if cs50.len() > 1 {
        let  pset = &cs50[1].to_lowercase();
        match pset.trim() {
            // psets to run
            "hello" => {intro(&cs50); hello::play()}, 
            "mario" => {intro(&cs50); mario_more::play()}, 
            "caesar" => {intro(&cs50); caesar::play()}, 
            "credit" => {intro(&cs50); credit::play()},
            "crack" => {intro(&cs50); crack::play()},
            "vigenere" =>{intro(&cs50); vigenere::play()}, 
            _ => { writeln!(std::io::stderr(),"see README.md file for list of pset names \n https://github.com/saebs/cs50rs/blob/master/README.md").unwrap(); process::exit(1)}, //,
        } 
    } else{
        writeln!(std::io::stderr(), r##"Usage: cs50rs <pset name> 
"see README.md file for list of pset names"##).unwrap();
    }

// helper function

fn intro(cs: &Vec<String>){
    println!("\n\u{25b6}  {}" , cs[1].to_uppercase()); // play button
}
}

//ZOU: APL189395
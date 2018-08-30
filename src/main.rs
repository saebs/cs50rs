/* The Rust implementation of cs50 psets
// Student: Sabelo Ntabeni
// CS50 2018
// Instructor: David J Malan
*/
// The main.rs file is custom, commandline arguments will invoke the required module or pset to run
// for psets with further arguments , inputs start from argument three ie index 2
mod hello; // pset1/hello
mod mario_more; // pset1/mario/more
mod caesar; // pset2/caesar
mod credit; // pset2/credit
mod crack; // pset2/credit
use hello::hello_;
use mario_more::mario_;
use caesar::caesar_;
use credit::credit_;
use crack::crack_;

use std::env;
use std::process;

fn main() {
    let cs50: Vec<String> = env::args().collect();
    if cs50.len() > 1 {
        let  pset = &cs50[1].to_lowercase();
        match pset.trim() {
            // psets to run
            "hello" => {intro(&cs50); hello_()}, 
            "mario" => {intro(&cs50); mario_()}, 
            "caesar" => {intro(&cs50); caesar_()}, 
            "credit" => {intro(&cs50); credit_()},
            "crack" => {intro(&cs50); crack_()},
            _ => { eprintln!("see README.md file for list of pset names \n https://github.com/saebs/cs50rs/blob/master/README.md"); process::exit(1)}, //,
        } 
    } else{
        eprintln!("Usage: {} \"pset\" ", cs50[0]);
    }

// helper function

fn intro(cs: &Vec<String>){
    println!("\n\u{25b6}  {}" , cs[1].to_uppercase()); // play button
}
}


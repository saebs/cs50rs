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
use hello::hello;
use mario_more::mario;
use caesar::caesar;
use credit::credit;
use crack::crack;

use std::env;
use std::process;
use std::io::Write;

fn main() {
    let cs50: Vec<String> = env::args().collect();
    if cs50.len() > 1 {
        let  pset = &cs50[1].to_lowercase();
        match pset.trim() {
            // psets to run
            "hello" => {intro(&cs50); hello()}, 
            "mario" => {intro(&cs50); mario()}, 
            "caesar" => {intro(&cs50); caesar()}, 
            "credit" => {intro(&cs50); credit()},
            "crack" => {intro(&cs50); crack()},
            _ => { writeln!(std::io::stderr(),"see README.md file for list of pset names \n https://github.com/saebs/cs50rs/blob/master/README.md").unwrap(); process::exit(1)}, //,
        } 
    } else{
        writeln!(std::io::stderr(), "Usage: cs50rs pset [option]").unwrap();
    }

// helper function

fn intro(cs: &Vec<String>){
    println!("\n\u{25b6}  {}" , cs[1].to_uppercase()); // play button
}
}

//ZOU: APL189395
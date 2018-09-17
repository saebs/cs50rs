/* The Rust implementation of cs50 psets
// Student: Sabelo Ntabeni
// CS50 2018
// Instructor: David J Malan
*/
// The main.rs file is custom, commandline arguments will invoke the required module or pset to run
// for psets with further arguments , inputs start from argument three ie index 2
mod pset_01; // hello, mario(more comfortable), credit
mod pset_02; // caesar, crack(pending calling c code lesson)

// Problem sets
use pset_01::hello;
use pset_01::mario_more;
use pset_01::credit;
use pset_02::caesar;
use pset_02::crack;
// other 
use std::env;
use std::process;
use std::io::Write;

fn main() {
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
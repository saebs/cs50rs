/* The Rust implementation of cs50 psets
// Student: Sabelo Ntabeni
// CS50 2018
// Instructor: David J Malan
*/
// The main.rs file is custom, commandline arguments will invoke the required module or pset to run
// for psets with further arguments , inputs start from argument three ie index 2
mod hello; // pset1/hello
mod mario_more; // pset1/mario/more
use hello::halo;
use mario_more::mario;
use std::env;


fn main() {
    let pset: Vec<String> = env::args().collect();
    let  arg = &pset[1].to_lowercase();
    println!("{}" ,arg);
    match arg.trim() {
        // psets to run
        "hello" => halo(), // pset1/hello
        "mario" => mario(), // pset1/mario/more
        _ => println!("nothing to do or check speling for program to run"),
    } 
}

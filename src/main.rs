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
use hello::hello_;
use mario_more::mario_;
use caesar::caesar_;
use std::env;


fn main() {
    let cs50: Vec<String> = env::args().collect();
    if cs50.len() > 1 {
        let  pset = &cs50[1].to_lowercase();
        println!("\n\u{25b6}  {}" , pset.to_uppercase()); // play button
        println!("");
        match pset.trim() {
            // psets to run
            "hello" => hello_(), // pset1/hello
            "mario" => mario_(), // pset1/mario/more
            "caesar" => caesar_(), //pset2/caesar
            _ => println!("nothing to do or check spelling for program to run"),
        } 
    } else{
        println!("Usage: {} \"pset\" ", cs50[0]);
    }
}

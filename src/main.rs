/* The Rust implementation of cs50 psets
// Student: Sabelo Ntabeni
// CS50 2018
// Instructor: David J Malan
*/
// The main.rs file is custom, commandline arguments will invoke the required module or pset to run
// for psets with further arguments , inputs start from argument three ie index 2
// other 
extern crate cs50rs;
use std::env;
use cs50rs::run;
use std::io::Write;
use std::process;
fn main() {
    let cs50: Vec<String> = env::args().collect();
    if cs50.len() < 2 {
        help();
    } else {
        let args = &cs50[1].to_lowercase();
        if args.trim() != "help" {
            println!("\u{25b6}\n{} running\n", &cs50[1].to_uppercase());
            run(&cs50);
        }
    } 
}

fn help() {
             writeln!(std::io::stderr(),
r##"Usage.. cs50rs "pset name"

[Options]
"hello" 
"mario" 
"caesar" 
"credit" 
"crack" 
"vigenere"
"##).unwrap(); process::exit(1) 

}
//ZOU: APL189395
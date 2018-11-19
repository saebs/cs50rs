mod pset1_rust;
mod pset2_crypto;

use pset1_rust::*; // aka C
use pset2_crypto::*; 
use std::process;
use std::io::Write;

// pub enum Pset {
//     Hello,
//     Mario,
//     Caesar,
//     Credit,
//     Crack,
//     Vigenere,
// }

// pub trait Start {
//     fn play(&self) {}
// }


pub fn run(query: &Vec<String>) {
        let  pset = &query[1].to_lowercase();
        match pset.trim() {
            // psets to run
            "hello" => hello::play(), 
            "mario" => mario_more::play(), 
            "caesar" => caesar::play(), 
            "credit" => credit::play(),
            "crack" => crack::play(),
            "vigenere" => vigenere::play(), 
            _  => { writeln!(std::io::stderr(),"Something went wrong Or pset not implemented yet!!").unwrap(); process::exit(1)}, //,
        } 
}


//ZOU: APL189395
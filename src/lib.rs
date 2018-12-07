pub mod pset1_rust;
pub mod pset2_crypto;

use crate::pset1_rust::*; // aka C
use crate::pset2_crypto::*; 
use std::process;
use std::io::Write;

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
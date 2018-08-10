use std::env;
use std::process;

pub fn caesar_(){
    let args: Vec<String> = env::args().collect();
    // this custom implementation will require three arguments e.g:process
    // ./cs50rs -pset-name  --optional-pset-argument
    // there minimun arg length is 2 
    if args.len() == 2 || args.len() != 3 { 
        println!("Usage: {} {} k", args[0], args[1]);
        process::exit(1);
    }
}
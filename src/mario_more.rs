// The Rust implementation of cs50 pset1 mario
// Student: Sabelo Ntabeni
// CS50 2018
// Instructor: David J Malan
// pset1/mario/more
use std::io;

pub fn mario() {
    let n: i32;
    loop {
        println!("Height:");
        let mut height = String::new();
        io::stdin().read_line(&mut height).expect("Failed");
        let height: i32 = match height.trim().parse() {
            Ok(ht) => ht,
            Err(_) => continue,
        };  
        if !(height < 0 || height > 23) { n = height; break;} 
    }
    // drawing pyramid
    // Rnth row is defined :
    //  R = n(height - n)(spaces) + n(blocks) + 2spaces + n(blocks) + line break
    // let r = n , for n>0
    for row in 1..=n {
        //draw left half of pyramid
        // padding from left edge
        for _left_spaces in 0..(n - row) {
            print!("{}",' ');
        }
        // left set of blocks 
        for _blocks in 0..row {
            print!("\u{2593}");
        }
        // gap between left and right pyramid halves
        print!("  ");
        // right pyramid half
        for _blocks in 0..row {
            print!("\u{2593}");
        }
        // pyramid level finished
        println!(); 
    }
} 
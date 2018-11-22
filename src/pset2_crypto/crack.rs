// Pset relies on crypt function writteno in C, To be immplemented after learning to call C code in rust.
use std::process;

extern "C" {
    #[repr(C)]
    fn crypt<'a>(key: *const u8, salt: *const u8) -> *const u8; 
}


pub fn play() {
/* unsafe {
    let ciphertext = crypt('a', 50); // fdup , I'll be back when i have a black belt
    println!("{:?}", ciphertext);
} */
let alphabet = vec!["a","b","c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
let mut gen_word = String::new();
    for &first in alphabet.iter() {
        gen_word = format!("{}", first );
        // assert_eq!(String::from("a"), gen_word );
        println!("{}", gen_word);
        for &second in alphabet.iter() {
            gen_word = format!("{}{}",first, second );
                println!("{}", gen_word);
            for &third in alphabet.iter() {
                gen_word = format!("{}{}{}",first, second, third );
                println!("{}", gen_word);
                for &fourth in alphabet.iter() {
                    gen_word = format!("{}{}{}{}", first, second, third, fourth);
                    println!("{}", gen_word);
                    for &fifth in alphabet.iter() {
                        gen_word = format!("{}{}{}{}{}",first, second, third, fourth, fifth);
                        println!("{}", gen_word);
                    }
                }
            }
        }
    }
}
// #[allow(dead_code)]
// fn check(word: &str) {
//     if word == crypt(pass, salt) {
//         println!("itsho");
//         process::exit(1);
//     }
// }
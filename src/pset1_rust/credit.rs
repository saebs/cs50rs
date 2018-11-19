/* The Rust implementation of cs50 pset1 credit
Student: Sabelo Ntabeni
CS50 2018
Instructor: David J Malan
pset1: credit
Implement a program that determines whether a provided credit card number is valid according to Luhn’s algorithm.
*/

use std::process;
use std::io;
use std::io::Write;
// use Pset;
// use Start;


pub    fn play() {
        //  standard credit card number sizes
        const SXTN: usize = 16;
        const FFTN: usize = 15;
        const THTN: usize = 13;

        const M: &str = "MASTERCARD";
        const V: &str = "VISA";
        const A: &str = "AMEX";
        const Z: &str = "ZIMSWITCH";

        let mut card_number: String;
        // let mut raw_input = String::new();
        print!("Number: "); 
        let _ = io::stdout().flush();
        card_number = String::new();
        io::stdin().read_line(&mut card_number).expect("failed input"); 
        
        let digits: Vec<char> = card_number.trim().chars().collect(); // trim raw input bro
        // eprintln!("digits{:?}", digits);
        let mut mult = false; // to multiply or not to,
        let mut sum = 0;
        for each in &mut digits.iter().rev() {
            let digit: u32 = match each {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                '0' => 0,
                _ => 0,
            };
            //  
            sum += if mult {
                mult = false;
                // separate double digit product and add both to sum
                let units = (digit * 2) % 10;
                // eprintln!("{}", units);
                let tens = (digit * 2) / 10;
                // eprintln!("{}", tens);
                units + tens 
            } else {
                mult = true;
                digit
            }; 
        }
        // eprintln!("{}", sum);
        // checksum validation
        if sum%10 != 0 {
            println!("INVALID");
            process::exit(1);
        }

        // Validate credit card provider
        // AMEX , VISA, MASTER CARD, and including ZIMSWITCH :-)
        // dj ganyani song ,- Q love

        let mii = digits[0]; // aka Major Industry Identifier 
        let second_digit = digits[1];
        let (card_issuer, _, _ ) = match (mii, second_digit, digits.len()) {
            ('5', '1'...'5', SXTN) =>  (M, {}, {}),  // Mastercard numbers are 16 digit, start with 51 to 55
            ('4', _, THTN) | ('4', _, SXTN) => (V, {},  {}),  // Visa numbers are 13 or 16 digit, all start with 4
            ('6', '0', SXTN ) | ('5', '8', SXTN)=> (Z, {} , {}),   // Zimwsitch numbers are unknoen to me for now just use the ones i have 
            ('3', '4' , FFTN) | ('3', '7', FFTN) => (A, {}, {}),  // American express number are 15 digit, start with 34 or 37
            _ => ("INVALID", {}, {}),
            // dj ganyani - Q love
        };
        println!("{}", card_issuer);  
    } 
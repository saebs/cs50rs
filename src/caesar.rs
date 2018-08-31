/* Rust implimentation of CS50 pset2/caesar
student: Sabelo Ntabeni
instruction: write a program that encrypts text using caesars cipher
*/
use std::env;
use std::process;
use std::io;
use std::io::Write;
const LETTERS:  i32 = 26;
const UC: u8 = 65;
const LC: u8 = 97;

pub fn caesar(){
    let args: Vec<String> = env::args().collect();
    // this custom implementation will require three arguments e.g:process
    // ./cs50rs -pset-name  --optional-pset-argument
    // there minimun arg length is 2 
    if args.len() == 2 || args.len() != 3 { 
        writeln!(io::stderr(), "Usage:: cs50rs caesar k").unwrap();
        process::exit(1);
    }

    // get key int
    let key: i32 = match args[2].trim().parse() {
        Ok(k) => k,
        Err(_) => 0, 
    };
    // prompt for plain text
    // eprintln!("{}", key);
    print!("plaintext: ");
    let _ = io::stdout().flush();
    let mut plaintxt = String::new();
    match io::stdin().read_line(&mut plaintxt) {
        Ok(_) => {},
        Err(err) => println!("{}",err),
    };
    let plaintxt = plaintxt.as_str();
    print!("\nciphertext: ");
    // loop over each xcter and preserve case, encrypt iff alphabetic , preserve if non-alphabetic 
    for p in plaintxt.chars() {
        // check for case and get alphabetic index
        // ascii-code % a or A ascii code point = alphabetic index
        if p.is_alphabetic(){
            let (pi, shift) = if p.is_uppercase() {
                (p as u8 % UC, UC)
            } else {
                (p as u8 % LC, LC)
            };
            // encrypt current xter
            let c = {
                cipher(pi , key) + shift
            };
            // print!("pi {} shift {}", pi, shift);
            print!("{}", c as char);
        } else {
            print!("{}",p);
        }
    }
    print!("\n");
}

fn cipher(pi: u8, key: i32) -> u8 {
    ((pi as i32 + key ) % LETTERS) as u8
}
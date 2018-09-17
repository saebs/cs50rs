// Pset relies on crypt function writteno in C, To be immplemented after learning to call C code in rust.
use std::time::Instant;
use std::process;


pub fn play() {
    let track = Instant::now();
    eprintln!("{:?}",track.elapsed() );
let alphabet = ['a','b','c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut k = 0;
    for &f in alphabet.iter() {
        println!("{}", f );
        k += 1;
        for &g in alphabet.iter() {
            println!("{}{}",f, g );
            k +=1;
            for &h in alphabet.iter() {
                println!("{}{}{}",f,g,h );
                k += 1;
                // if f == 'a' && g == 'd' && h == 'd'{
                //     eprintln!("tolakele! at {:?}", track.elapsed());
                //     process::exit(1);
                // }
                for &i in alphabet.iter() {
                    println!("{}{}{}{}", f, g, h, i);
                    if f == 'm' && g == 'o' && h == 'o' && i == 'n'{
                        eprintln!("tolakele! at {:?}", track.elapsed());
                        process::exit(1);
                    }
                    k +=1;
                    for &j in alphabet.iter() {
                        println!("{}{}{}{}{}",f, g, h, i, j);
                    }
                }
            }
        }
    }
    eprintln!("permutations {}", k);
    eprintln!("{:?}", track.elapsed());
}
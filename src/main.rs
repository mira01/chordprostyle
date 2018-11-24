extern crate chordprostyle;
use chordprostyle::lex;

use std::io::Read;
use std::env;
use std::fs::File;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    
    println!("I got {:?} args: {:?}", args.len()-1, &args[1..]);
    println!("contents {}", &mut contents);

    let chars = contents.chars();
    let result = lex(chars);
    for a in result{
        println!("a: {:?}", a);
    }

//    for ch in chars{
//        println!("ch: {:?}", ch);
//    }
}

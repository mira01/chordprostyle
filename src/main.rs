extern crate chordprostyle;
use chordprostyle::lex;
use chordprostyle::HtmlFormatter;

use std::io::Read;
use std::env;
use std::fs::File;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);

    let chars = contents.chars();
    let result = lex(chars);
    let formater = HtmlFormatter::new(result, "");
    let res = formater.format();
    println!("{:?}", res);
    //for a in result{
    //    println!("{:?}", a);
    //}

}

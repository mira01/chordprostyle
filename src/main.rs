extern crate chordprostyle;
use chordprostyle::{lex, parse};
use chordprostyle::HtmlFormatter;
use chordprostyle::PdfFormatter;

use std::io::Read;
use std::env;
use std::fs::File;

extern crate printpdf;
use printpdf::*;
use std::io::BufWriter;


fn main(){
    let args = env::args().skip(1);
    //println!("<html><head><link rel='stylesheet' href='styl.css'></head><body>");
    for path in args{
        process_file(path);
    }
    //println!("<div class='footer'>...</div></body></html>");
}

fn process_file(path: String){
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);

    let chars = contents.chars();
    let result = lex(chars);
    let parser = parse(result);
    parser.parse();
    //let formater = PdfFormatter::new(result);
    //let formater = HtmlFormatter::new(result, "styl.css");
    //let res = formater.format();
    //println!("{}", res);

}

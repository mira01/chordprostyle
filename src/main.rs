extern crate chordprostyle;
use chordprostyle::lexer::lex;
use chordprostyle::parser::parse;

use std::io::Read;
use std::env;
use std::fs::File;

fn main(){
    let args = env::args().skip(1);
    for path in args{
        process_file(path);
    }
}

fn process_file(path: String){
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    let chars = contents.chars();
    let lexresult = lex(chars);
    let lex_only = false;

    if lex_only {
        for token in lexresult{
            println!("{:?}", token);
        }
    }else{
        let mut parser = parse(lexresult);
        let res = parser.parse();

        //let formater = PdfFormatter::new(lexresult);
        //let formater = HtmlFormatter::new(lexresult, "styl.css");
        
 //       let formater = ParseFormatter::new(res);
 //       let res = formater.format();
 //       println!("{:?}", res);
    }
}

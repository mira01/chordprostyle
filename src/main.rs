extern crate chordprostyle;
use chordprostyle::lexer::lex;
//use chordprostyle::parser::parse;
use chordprostyle::tri_parser::parse;
use chordprostyle::formatters::parse_formatter::ParseFormatter;

use std::io::Read;
use std::env;
use std::fs::File;

fn main(){
    let args = env::args().skip(1);
    for path in args{
        match process_file(&path) {
            Some(song) =>{
                let formater = ParseFormatter::new(song);
                let res = formater.format();
                println!("{}", res);
            },
            None =>{
                println!("song {} error", path);
            }
        }
    }
}

fn process_file(path: &String) -> Option<chordprostyle::model::Song>{ //better be Result
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
        None
    }else{
        let mut parser = parse(lexresult);
        let song = parser.parse();

        Some(song)
    }
}

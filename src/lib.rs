pub mod lexer;
pub mod parser;
pub mod formatters;
pub mod model;
pub mod tri_parser;

use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use formatters::parse_formatter::ParseFormatter;

pub fn process_files<T>(paths: T) where T: Iterator<Item=String>{
    println!("<html><head><link rel='stylesheet' href='styl5.css'><meta charset='utf-8'></head><body>");
    for (i, path) in paths.enumerate(){
        match process_file(&path) {
            Some(song) =>{
                let formater = ParseFormatter::new(song);
                let res = formater.format(&(i+1).to_string());
                println!("{}", res);
            },
            None =>{
                eprintln!("song {} error", path);
            }
        }
    }
    println!("</body></html>");
}

fn process_file(path: &String) -> Option<model::Song>{ //better be Result
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);

    let chars = contents.chars();
    let lexresult = lexer::lex(chars);
    let lex_only = false;

    if lex_only {
        for token in lexresult{
            println!("{:?}", token);
        }
        None
    }else{
        let mut parser = tri_parser::parse(lexresult);
        let song = parser.parse();

        Some(song)
    }
}

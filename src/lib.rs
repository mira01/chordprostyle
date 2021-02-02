pub mod lexer;
pub mod formatters;
pub mod model;
pub mod tri_parser;

use std::io::Read;
use std::fs::File;
use std::str::Chars;

use formatters::parse_formatter::ParseFormatter;
use model::Song;
use crate::lexer::Lexer;

pub trait Parser{
    fn parse(&mut self, chars: Chars) -> Result<Song, String>;
}
pub trait Formatter{
    fn pre(&self, context: &mut Context) -> String;
    fn format(&self, song: Song, context: &mut Context) -> String;
    fn post(&self, context: &mut Context) -> String;
}

pub struct Context{
    number: usize,
}
impl Context{
    pub fn new() -> Context{
        Context{
            number: 0,
        }
    }

    pub fn next_number(&mut self) -> usize{
        self.number += 1;
        self.number
    }
}

pub fn process_files2<I, P, F>(paths: I, parser: P, formatter: F)
    where I: Iterator<Item=String>,
          P: Parser,
          F: Formatter,
{
    let mut context = Context::new();
    println!("{}",formatter.pre(&mut context));
    for (i, path) in paths.enumerate(){
        match process_file(&path) {
            Ok(song) =>{
                let res = formatter.format(song, &mut context);
                println!("{}", res);
            },
            Err(_e) =>{
                eprintln!("song {} error", path);
            }
        }
    }

    println!("{}",formatter.post(&mut context));

}

fn process_file(path: &String) -> Result<model::Song, String>{ //better be Result
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let chars = contents.chars();
    let mut parser = tri_parser::TriParser::new();
    parser.parse(chars)
}

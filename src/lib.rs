pub mod lexer;
pub mod formatters;
pub mod model;
pub mod tri_parser;

use std::io::Read;
use std::fs::File;
use std::str::Chars;

use model::Song;

pub trait Parser{
    fn parse(&mut self, chars: Chars) -> Result<Song, String>;
}
pub trait Formatter{
    fn pre(&self, context: &mut Context) -> String;
    fn format(&self, song: Song, context: &mut Context) -> String;
    fn post(&self, context: &mut Context) -> String;
}

#[derive(Default)]
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

pub fn process_files<I, P, F>(paths: I, parser: &mut P, formatter: F)
    where I: Iterator<Item=String>,
          P: Parser,
          F: Formatter,
{
    let mut context = Context::new();
    println!("{}",formatter.pre(&mut context));
    for path in paths{
        match parse_file(&path, parser) {
            Ok(song) => {
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

fn parse_file(path: &str, parser: &mut dyn Parser) -> Result<model::Song, String>{
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let chars = contents.chars();
    parser.parse(chars)
}

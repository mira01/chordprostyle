pub mod lexer;
pub mod formatters;
pub mod model;
pub mod tri_parser;

use std::io;
use std::fs;
use std::error;
use std::str::Chars;

use model::Song;

type ParseResult = Result<Song, LibError>;

pub trait Parser{
    fn parse(&mut self, chars: Chars) -> ParseResult;
}

type FormatResult = Result<String, LibError>;

pub trait Formatter{
    fn pre(&self, context: &mut Context) -> String;
    fn format(&self, song: Song, context: &mut Context) -> FormatResult;
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

#[derive(Debug)]
#[non_exhaustive]
pub enum LibError{
    IOError(io::Error),
    ParseError(),
    FormatError(),
}

impl From<io::Error> for LibError{
    fn from(error: io::Error) -> Self{
        LibError::IOError(error)
    }
}

pub fn process_files<I, P, F>(paths: I, parser: &mut P, formatter: F) -> Result<(), Vec<(String, LibError)>>
    where I: Iterator<Item=String>,
          P: Parser,
          F: Formatter,
{
    let mut context = Context::new();
    let mut errors = Vec::new();
    println!("{}",formatter.pre(&mut context));
    for path in paths{
        if let Ok(output) = parse_file(&path, parser)
            .and_then(|song|{
                formatter.format(song, &mut context)
            })
            .map_err(|e|{errors.push((path, e))})
        {
            println!("{}", output);
        }
    }

    println!("{}",formatter.post(&mut context));
    match errors.is_empty(){
        true => Ok(()),
        false => Err(errors),
    }
}

fn parse_file(path: &str, parser: &mut dyn Parser) -> Result<Song, LibError>{
    let contents = fs::read_to_string(path)?;
    let chars = contents.chars();
    parser.parse(chars)
}

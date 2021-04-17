pub mod lexer;
pub mod formatters;
pub mod model;
pub mod tri_parser;

use std::io;
use std::fs;
use std::error::Error;
use std::fmt::Display;
use std::str::Chars;

use std::any::Any;
use std::collections::HashMap;

use model::Song;

type ParseResult = Result<Song, LibError>;

/// Trait for parsers returning a possible Song
pub trait Parser{
    fn parse(&mut self, chars: Chars) -> ParseResult;
}

type FormatResult = Result<String, LibError>;

/// Trait that formats Songs into Strings
pub trait Formatter<T: Context>{
    fn pre(&self, context: &mut T) -> FormatResult;
    fn format(&self, song: Song, context: &mut T) -> FormatResult;
    fn post(&self, context: &mut T) -> FormatResult;
}

/// Trait defining container for custom data.
///
/// Formatters may want to save and retrieve some data while formatting songs.
/// Context provides ability to do it.
pub trait Context{
    fn set(&mut self, key: &str, value: Box<dyn Any + 'static>) -> ();
    fn get<T: 'static>(&self, key: &str) -> Option<&T>;
}

#[derive(Default)]
pub struct KeyValueStore{
    map: HashMap<String, Box<dyn Any + 'static>>,
}
impl KeyValueStore{
    pub fn new() -> KeyValueStore{
        KeyValueStore{
            map: HashMap::new(),
        }
    }
}

impl Context for KeyValueStore{
    fn set(&mut self, key: &str, value: Box<dyn Any + 'static>) -> (){
        self.map.insert(key.to_owned(), value);
    }
    fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.map.get(key).map(|boxed|{
            (*boxed).downcast_ref::<T>().unwrap()
        })
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum LibError{
    IOError(io::Error),
    ParseError(),
    FormatError(Box<dyn Error>),
}

impl Error for LibError{ }

impl Display for LibError{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>{
        match &self {
            LibError::IOError(ioerror) => write!(fmt, "{}", ioerror),
            _ => write!(fmt, "{:?}", &self),
        }
    }
}

impl From<io::Error> for LibError{
    fn from(error: io::Error) -> Self{
        LibError::IOError(error)
    }
}

/// Takes file-paths iterator, parses the file using parser and then formats the result
/// WARNING this function is not cosidered stable API
pub fn process_files<I, P, F, C>(paths: I, parser: &mut P, formatter: F, mut context: &mut C) -> Result<(), Vec<(String, LibError)>>
    where I: Iterator<Item=String>,
          P: Parser,
          C: Context,
          F: Formatter<C>,
{
    let mut errors = Vec::new();
    match formatter.pre(context){
        Ok(output) => print!("{}", output),
        Err(e) => errors.push((".".into(), e)),
    }
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

    match formatter.post(&mut context){
        Ok(output) => print!("{}", output),
        Err(e) => errors.push((".".into(), e)),
    }
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

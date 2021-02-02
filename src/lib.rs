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
    fn pre(&self, context: Context) -> String;
    fn format(&self, song: Song, context: Context) -> String;
    fn post(&self, context: Context) -> String;
}

pub struct Context{}

pub fn process_files2<I, P, F>(paths: I, parser: P, formatter: F)
    where I: Iterator<Item=String>,
          P: Parser,
          F: Formatter,
{
    let context = Context{};
    println!("{}",formatter.pre(Context{}));
    println!("{}",formatter.post(Context{}));

}

//pub fn process_files<T>(paths: T) where T: Iterator<Item=String>{
//    println!("<html><head><link rel='stylesheet' href='styl5.css'><meta charset='utf-8'></head><body>");
//    for (i, path) in paths.enumerate(){
//        match process_file(&path) {
//            Some(song) =>{
//                let formater = ParseFormatter();
//                let res = formater.format(&(i+1).to_string());
//                println!("{}", res);
//            },
//            None =>{
//                eprintln!("song {} error", path);
//            }
//        }
//    }
//    println!("</body></html>");
//}

fn process_file(path: &String) -> Result<model::Song, String>{ //better be Result
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let chars = contents.chars();
    let mut parser = tri_parser::TriParser::new();
    parser.parse(chars)
}

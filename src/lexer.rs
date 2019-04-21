use crate::model::{SongPart, DirectiveType};

use std::str::Chars;
use std::iter::Peekable;
//use std::iter::Iterator;
//use std::fs::File;
//use std::io::BufWriter;
//use std::fmt;

pub fn lex(song : Chars) -> Lexer {
    Lexer::new(song)
}

#[derive(Debug)]
pub struct Lexer<'a>{
    stream: Peekable<Chars<'a>>,
    state: SongPart,
}

impl<'a> Lexer<'a> {
    fn new(stream: Chars) -> Lexer{
        Lexer{
            stream: stream.peekable(),
            state: SongPart::Empty,
        }
    }

    fn lexit(&mut self, curr_char: char) -> Option<SongPart> {
        let res = match curr_char{
            '{' => {
                let mut directive = String::new();
                for c in self.stream.by_ref().take_while(|ch| *ch != '}'){
                    directive.push(c)
                }
                Some(self.lex_directive(&directive))

            },
            '[' => {
                let mut directive = String::new();
                for c in self.stream.by_ref().take_while(|ch| *ch != ']'){
                    directive.push(c)
                }
                Some(SongPart::Chord(directive))

            },
            '\n' =>
                Some(SongPart::NewLine),
            other => {
               let mut text = String::new();
               text.push(other);
               while let Some(&c) = self.stream.peek(){
                    match c {
                        '[' | '{' | '\n' => {break}
                        _ => {
                            text.push(self.stream.next().unwrap()) // always Some due to peek
                        }
                    }
               }
               Some(SongPart::Text(text))
            },
        };
        res
    }

    fn lex_directive(&mut self, directive: &str) -> SongPart{
       let parts: Vec<&str> = directive.split(':').collect();
       let (dir_type, value): (String, String) = if parts.len() > 1{
           (parts[0].to_string(), parts[1..].join(""))
       }else{
           (parts[0..].join(""), "".into())
       };
       let directive_type = match (dir_type.as_ref(), value) {
            ("ns", value) | ("new_song", value) => DirectiveType::NewSong,
            ("t", value) | ("title", value) => DirectiveType::Title(value.into()),
            ("soc", value) | ("start_of_chorus", value) => DirectiveType::ChorusStart,
            ("eoc", value) | ("end_of_chorus", value) => DirectiveType::ChorusEnd,
            ("c", value) | ("comment", value) => DirectiveType::Comment(value.into()),
            (t, value) => DirectiveType::Other(t.to_string()),
       };
       SongPart::Directive(directive_type)
    }

}

impl<'a> Iterator for Lexer<'a>{
    type Item = SongPart;

    fn next(&mut self) -> Option<SongPart>{
        let ch = self.stream.next();
        match ch{
            Some(character) => self.lexit(character),
            None => None
        }
    }
}

use crate::model::{SongPart, DirectiveType};

use std::str::Chars;
use std::iter::Peekable;

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
            ("ns", _value) | ("new_song", _value) => DirectiveType::NewSong,
            ("t", value) | ("title", value) => DirectiveType::Title(value.into()),
            ("soc", _value) | ("start_of_chorus", _value) => DirectiveType::ChorusStart,
            ("eoc", _value) | ("end_of_chorus", _value) => DirectiveType::ChorusEnd,
            ("c", value) | ("comment", value) => DirectiveType::Comment(value.into()),
            (t, _value) => DirectiveType::Other(t.to_string()),
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

#[cfg(test)]
mod tests{
    use crate::model::{SongPart, DirectiveType};
    use super::*;

    #[test]
    fn test_lex(){
        let mut contents = String::from("{ns}
{t: SongTitle}
[C]v jedne [Am]morske [Dm7]pustine
ztroskotal parnik v hlubine

{soc}
[C]ryba roba [Am]ryba roba [Dm]ryba roba [G] cucu
{eoc}");
        let chars = contents.chars();
        let mut lexresult = super::lex(chars);
        assert_eq!(lexresult.next().unwrap(), SongPart::Directive(DirectiveType::NewSong));
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);

        assert_eq!(lexresult.next().unwrap(), SongPart::Directive(DirectiveType::Title(" SongTitle".to_string())));
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);

        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("C".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text("v jedne ".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("Am".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text("morske ".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("Dm7".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text("pustine".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);

        assert_eq!(lexresult.next().unwrap(), SongPart::Text("ztroskotal parnik v hlubine".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);

        assert_eq!(lexresult.next().unwrap(), SongPart::Directive(DirectiveType::ChorusStart));
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);

        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("C".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text("ryba roba ".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("Am".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text("ryba roba ".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("Dm".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text("ryba roba ".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Chord("G".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::Text(" cucu".to_string()));
        assert_eq!(lexresult.next().unwrap(), SongPart::NewLine);

        assert_eq!(lexresult.next().unwrap(), SongPart::Directive(DirectiveType::ChorusEnd));

        assert!(lexresult.next().is_none());

    }
}

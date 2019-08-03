use std::str::Chars;

#[derive(Debug)]
pub enum SongPart{
    Text(String),
    Chord(String),
    Directive(String),
    Empty,
}

pub fn lex(song: Chars) -> Lexer {
    Lexer::new(song)
}
#[derive(Debug)]
pub struct Lexer<'a>{
    stream: Chars<'a>,
    state: SongPart,
}

impl<'a> Lexer<'a> {
    fn new(stream: Chars) -> Lexer{
        Lexer{
            stream: stream,
            state: SongPart::Empty,
        }
    }
}

impl<'a> Iterator for Lexer<'a>{
    type Item = SongPart;

    fn next(&mut self) -> Option<SongPart>{
        let ch = self.stream.next();
        match ch{
            Some(character) => Some(SongPart::Text(character.to_string())),
            None => None
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

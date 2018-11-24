use std::str::Chars;

pub enum SongPart{
    Text,
    Chord,
    Directive,
}

pub fn lex(song: Chars) -> Lexer {
    Lexer::new(song)
}
pub struct Lexer<'a>{
    stream: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(stream: Chars) -> Lexer{
        Lexer{
            stream: stream,
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

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

    fn lexit(&mut self, curr_char: char) -> Option<SongPart> {
        println!("curr_char: {:?}", curr_char);
        let res = match curr_char{
            '{' => {
                println!("jsem v zavorce");
                let mut directive = String::new();
                for c in self.stream.by_ref().take_while(|ch| *ch != '}'){
                    directive.push(c)
                }
                Some(SongPart::Directive(directive))

            },
            _ =>{
               Some(SongPart::Text(String::from("c")))
            },
        };
        res
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
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

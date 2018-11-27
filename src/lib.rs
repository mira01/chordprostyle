use std::str::Chars;
use std::iter::Peekable;
use std::iter::Iterator;

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
                Some(SongPart::Directive(directive))

            },
            '[' => {
                let mut directive = String::new();
                for c in self.stream.by_ref().take_while(|ch| *ch != ']'){
                    directive.push(c)
                }
                Some(SongPart::Chord(directive))

            },
            other => {
               let mut text = String::new();
               text.push(other);
               while let Some(&c) = self.stream.peek(){
                    match c {
                        '[' | '{' => {break}
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

pub struct HtmlFormatter<'a>{
    lexer: Lexer<'a>,
    stylesheet: &'a str, //TODO: fix injection
}

impl<'a> HtmlFormatter<'a>{
    pub fn new(lexer: Lexer<'a>, stylesheet: &'a str) -> HtmlFormatter<'a>{
        HtmlFormatter{
            lexer: lexer,
            stylesheet: stylesheet,
        }
    }
    pub fn format(self) -> String{
        let mut output = String::new();
        output.push_str(&String::from("<html><body>"));
        for part in self.lexer{
            match part{
                SongPart::Text(text) => {
                    output.push_str(&text);
                },
                SongPart::Directive(text) => {
                    output.push_str(&String::from("<h2>"));
                    output.push_str(&text);
                    output.push_str(&String::from("</h2>"));
                },
                SongPart::Chord(text) => {
                    output.push_str(&String::from("<span><strong>"));
                    output.push_str(&text);
                    output.push_str(&String::from("</strong></span>"));
                },
                _ => (),
            }
        }
        output.push_str(&String::from("</body></html>"));
        output
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

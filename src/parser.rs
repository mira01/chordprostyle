use crate::model::{SongPart, DirectiveType, VerseType, Song, Verse, Line};
use crate::lexer::{Lexer};

pub fn parse(lexer: Lexer) -> Parser {
    Parser::new(lexer)
}

pub struct Parser<'a>{
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a>{
    pub fn new(lexer: Lexer<'a>) -> Parser<'a>{
        Parser{lexer: lexer}
    }
    pub fn parse(&mut self) -> Song{
        let buffer = String::new();
        let song_title = match Parser::get_title(&mut self.lexer, buffer) {
            None => {
                eprintln!("pisnicka nema title");
                String::from("")
             }
            Some(a) => a
         };
        println!("song_title: {:?}", song_title);

        let verses = Parser::get_verses(&mut self.lexer);
        let song = Song{title: song_title, verses: verses};
        song
    }

    fn get_verses(lexer: &mut Lexer<'a>) -> Vec<Verse>{
        let is_interesting = |x: &SongPart| -> bool{
            match x {
                SongPart::Chord(_) => true,
                SongPart::Text(_) => true,
                SongPart::Directive(DirectiveType::Comment(_)) => true,
                SongPart::Directive(DirectiveType::ChorusStart) => true,
                _ => false
            }
        };
        let mut l2 = lexer.skip_while(|x|{!is_interesting(x)}).into_iter();

        //let mut song_parts = l2.into_iter().collect();
        let mut verses = Vec::<Verse>::new();
        while let Some(verse) = Parser::get_verse(&mut l2){
            verses.push(verse);
        }
        verses
    }

    fn get_verse(lexer: &mut Iterator<Item = SongPart>) -> Option<Verse>{
        let mut l3 = lexer.peekable();
        match l3.peek(){
            None => return None,
            _ => ()
        };
        let verse_type = {
            let lookin = l3.peek();
            match lookin{
                Some(SongPart::Directive(DirectiveType::ChorusStart)) => VerseType::Chorus,
                _ => VerseType::Common,
            }
        };
        let mut lines = Vec::<Line>::new();
        while let Some(line) = Parser::get_line(&mut l3){
            println!("line: {:?}", line);
            lines.push(line);
        }
        Some(Verse{verse_type: verse_type, lines: lines})
    }

    fn get_line(lexer: &mut Iterator<Item = SongPart>) -> Option<Line>{
        let mut l2 = lexer.peekable();
        let termination = |token: Option<&SongPart>| -> bool{
            match token {
                Some(SongPart::NewLine) => true,
                Some(SongPart::Directive(DirectiveType::ChorusStart)) => false,
                Some(SongPart::Directive(DirectiveType::ChorusEnd)) => true,
                None => true,
                _ => false,
            }
        };
        match l2.peek() {
            Some(SongPart::NewLine) => {
                let whatisit = l2.next();
                println!("whatisit: {:?}", whatisit);
                match l2.peek(){
                    Some(SongPart::NewLine) => {l2.next(); ()},
                     _ => ()
                }
                None
            },
            Some(SongPart::Directive(DirectiveType::ChorusEnd)) => {l2.next(); None},
            None => None,
            _ => {
                 let mut has_chords = false;
                 let mut line = Vec::<SongPart>::new();
                 while {!termination(l2.peek())}{
                    let token = l2.next().unwrap();
                    match token {
                         SongPart::Chord(_) => has_chords = true,
                         _ => (),
                    }
                    line.push(token);
                 }
                 Some(Line{has_chords, song_parts: line})
            }
       }
    }

    fn get_title<'b>(lexer: &mut Lexer<'a>, mut result: String) -> Option<String>{
        let mut l2 = lexer.peekable();
        //let mut a = Some(SongPart::Directive(DirectiveType::NewSong));
        let mut a = None;
        let mut found = false;
        while {
            if let Some(val) = l2.peek() {
                match val {
                    SongPart::Chord(_) | SongPart::Directive(DirectiveType::Comment(_)) | SongPart::Text(_) => {
                       true
                    },
                    SongPart::Directive(DirectiveType::Title(_title)) => {
                        found = true;
                        false
                    }
                    _ =>{
                        true
                    }
                }
            } else {
                false
            }
        } && !found
        {
            a = l2.next();
        };
        if found {
            a = l2.next();
        }
        println!("a: {:?}", a);
        match a {
            Some(SongPart::Directive(DirectiveType::Title(title))) => {
                result.push_str(&title);
                Some(result)
            },
            _ => None
        }
    }
}


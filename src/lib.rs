use std::str::Chars;
use std::iter::Peekable;
use std::iter::Iterator;
extern crate printpdf;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub enum DirectiveType{
    Title(String),
    NewSong,
    ChorusStart,
    ChorusEnd,
    Comment(String),
    Other(String),
}
#[derive(Debug)]
pub enum SongPart{
    Text(String),
    Chord(String),
    Directive(DirectiveType),
    NewLine,
    Empty,
}

pub fn lex(song: Chars) -> Lexer {
    Lexer::new(song)
}
pub fn parse(lexer: Lexer) -> Parser {
    Parser::new(lexer)
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

pub struct Parser<'a>{
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a>{
    pub fn new(lexer: Lexer<'a>) -> Parser<'a>{
        Parser{lexer: lexer}
    }
    pub fn parse(&mut self) -> (){
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
        let mut song = Song{title: &song_title, verses: verses};

        println!("song: {:?}", song);
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
        let mut song_parts = Vec::<SongPart>::new();
        Parser::get_verse(&mut l2);
        let line = Line{has_chords: true, song_parts: song_parts};
        let verse = Verse{verse_type: VerseType::Common, lines: vec![line]};
        vec![verse]
    }

    fn get_verse(lexer: &mut Iterator<Item = SongPart>) -> Verse{
        let mut l3 = lexer.peekable();
        let mut has_chords = false;
        let mut song_parts = Vec::<SongPart>::new();
        let verse_type = {
            let lookin = l3.peek();
            println!("lookin: {:?}", lookin);
            match lookin{
                Some(SongPart::Directive(DirectiveType::ChorusStart)) => VerseType::Chorus,
                _ => VerseType::Common,
            }
        };
        let line = Parser::get_line(&mut l3, &verse_type);
        println!("line: {:?}", line);
        Verse{verse_type: verse_type, lines: vec![line.unwrap()]}
    }

    fn get_line(lexer: &mut Iterator<Item = SongPart>, verse_type: &VerseType) -> Option<Line>{
        let mut l2 = lexer.peekable();
        let mut skip_new_lines = match verse_type{
            VerseType::Common => 0,
            VerseType::Chorus => 1,
        };
        let termination = |token: Option<&SongPart>, skip_new_lines: &mut i32| -> bool{
            match token {
                Some(SongPart::NewLine) =>{
                    *skip_new_lines -= &1;
                    skip_new_lines < &mut 0
                },
                Some(SongPart::Directive(DirectiveType::ChorusStart)) => false,
                Some(SongPart::Directive(DirectiveType::ChorusEnd)) => true,
                _ => false,
            }
        };
        let mut has_chords = false;
        let mut line = Vec::<SongPart>::new();
        while {!termination(l2.peek(), &mut skip_new_lines)}{
           let token = l2.next().unwrap();
           println!("token: {:?}", token);
           match token {
                SongPart::Chord(_) => has_chords = true,
                _ => (),
           }
           line.push(token);
        }
        Some(Line{has_chords, song_parts: line})
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
                    SongPart::Directive(DirectiveType::Title(title)) => {
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

#[derive(Debug)]
pub struct Song<'a>{
    pub title: &'a str,
    pub verses: Vec<Verse>,
}
#[derive(Debug)]
pub struct Verse{
    pub verse_type: VerseType,
    pub lines: Vec<Line>,
}
#[derive(Debug)]
pub enum VerseType{
    Common,
    Chorus,
}
#[derive(Debug)]
pub struct Line{
    pub has_chords: bool,
    pub song_parts: Vec<SongPart>,
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
        output.push_str(&String::from(format!("<div class='song'>")));
        for part in self.lexer{
            match part{
                SongPart::Text(text) => {
                    output.push_str(&format!("{}", &text));
                },
                SongPart::Directive(DirectiveType::Title(value))=> {
                    output.push_str(&format!("<h2>{}</h2>", &value));
                },
                SongPart::Directive(DirectiveType::ChorusStart)=> {
                    output.push_str(&format!("<div class='ref'>"));
                },
                SongPart::Directive(DirectiveType::ChorusEnd)=> {
                    output.push_str(&format!("</div>"));
                },
                SongPart::Directive(DirectiveType::Comment(value))=> {
                    output.push_str(&format!("<span class='comment'>{}</span>", value));
                },
                SongPart::Directive(value) => {
                    output.push_str(&String::from(format!("<div class='directive {:?}'>{:?}</div>", value, value)));
                },
                SongPart::Chord(text) => {
                    output.push_str(&format!("<span class='chord'><strong class='chord'>{}</span></strong>", text));
                },
                SongPart::NewLine =>{
                    output.push_str(&String::from("<br/>"));
                },
                _ => (),
            }
        }
        output.push_str(&String::from("</div>"));
        output
    }
}

pub struct PdfFormatter<'a>{
    lexer: Lexer<'a>
}
impl<'a> PdfFormatter<'a>{
    pub fn new(lexer: Lexer<'a>) -> PdfFormatter<'a>{
        PdfFormatter{lexer: lexer}
    }
    pub fn format(self) -> (){
        let doc_title = "Titulek";
        let (doc_width, doc_height) = (Mm(210.0), Mm(297.0));
        let (doc, page1, layer1) = PdfDocument::new(doc_title, doc_width, doc_height, "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let text = "Lorem ipsum";
        let text2 = "unicode: příliš žluťoučký kůň úpěl ďábelské ódy";

        let font2 = doc.add_external_font(File::open("/System/Library/Fonts/Palatino.ttc").unwrap()).unwrap();

        // text, font size, x from left edge, y from top edge, font
        ////current_layer.use_text(text, 14, Mm(20.0), Mm(20.0), &font2);

        // For more complex layout of text, you can use functions
        // defined on the PdfLayerReference
        // Make sure to wrap your commands
        // in a `begin_text_section()` and `end_text_section()` wrapper
        current_layer.begin_text_section();

            // setup the general fonts.
            // see the docs for these functions for details
            current_layer.set_font(&font2, 14);
            current_layer.set_text_cursor(Mm(10.0), Mm(270.0));
            current_layer.set_line_height(33);
            current_layer.set_word_spacing(3000);
            current_layer.set_character_spacing(10);
            //current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

            // write two lines (one line break)
            current_layer.write_text(text.clone(), &font2);
            current_layer.add_line_break();
            current_layer.write_text(text2.clone(), &font2);
            current_layer.add_line_break();

            // write one line, but write text2 in superscript
            current_layer.write_text(text.clone(), &font2);
            current_layer.set_line_offset(10);
            current_layer.write_text(text2.clone(), &font2);

    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
    //for a in result{
    //    println!("{:?}", a);
    //}

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

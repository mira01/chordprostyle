use std::str::Chars;
use std::iter::Peekable;
use std::iter::Iterator;
extern crate printpdf;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub enum SongPart{
    Text(String),
    Chord(String),
    Directive(String),
    NewLine,
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
        let text2 = "unicode: стуфхfцчшщъыьэюя";

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
        output.push_str(&String::from(format!("<html><head><link rel='stylesheet' href='{}'></head><body>", self.stylesheet)));
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
                SongPart::NewLine =>{
                    output.push_str(&String::from("<br/>"));
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

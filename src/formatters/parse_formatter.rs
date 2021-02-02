use crate::model::{SongPart, DirectiveType, VerseType, Song};
use crate::Context;
use crate::Formatter;
use crate::lexer::Lexer;


impl Formatter for ParseFormatter{
    fn pre(&self, _context: Context) -> String{
    "<html>
        <head><
        link rel='stylesheet' href='styl5.css'>
        <meta charset='utf-8'>
        </head>
        <body>".to_string()
    }

    fn format(&self, song: Song, _context: Context) -> String{
        "OBSAH".to_string()
    }

    fn post(&self, _context: Context) -> String{
        "</body></html>".to_string()
    }
}

pub struct ParseFormatter();

impl ParseFormatter{

    fn format_song_part(part: &SongPart) -> String{
        match part {
            SongPart::Text(t) => t.to_owned(),
            SongPart::Chord(t) => String::from(format!("<span class='chord'><strong class='chord'>{}</strong></span>", t.to_owned())),
            SongPart::Directive(DirectiveType::Comment(t)) => String::from(format!("<span class='comment'>{}</span>", t.to_owned())),
            _ => String::from("XXXXXX"),
        }
    }
    pub fn format_XXXXXXX(song: &Song, number: &str) -> String{
        let mut output = String::new();
        output.push_str(&String::from(
                format!("\n<div class='song'>\n\t<h1><span class='number'>{}</span>{}</h1>\n", &number, &song.title)));
        for ref verse in &song.verses{


         ////// V
            let chorus = match verse.verse_type{
                VerseType::Chorus => " ref",
                _ => "",
            };
        ////// ^



            output.push_str(&String::from(format!("\t<div class='verse {}'>\n", chorus)));
            {
                for ref line in &verse.lines{
                    let mut has_chords = "";
                    if line.has_chords{
                        has_chords = " has_chords";
                    }
                    output.push_str(&String::from(format!("\t\t<div class='line{}'>", has_chords)));
                    for song_part in &line.song_parts{
                        output.push_str(&String::from(format!("{}", ParseFormatter::format_song_part(song_part))));
                    }
                    output.push_str(&String::from(format!("</div>\n")));
                }
            }
            output.push_str(&String::from(format!("\t</div>\n")));
        }
        output.push_str(&String::from(format!("</div>\n")));
        output
    }
}
//impl fmt::Display for ParseFormatter {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "<h2>{}</h2>", &self.song.title).unwrap();
//        for verse in &self.song.verses{
//            write!(f, "<div class='sloka'>{:?}</div>", verse).unwrap();
//        }
//        Ok(())
//    }
//}

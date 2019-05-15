use crate::model::{SongPart, DirectiveType, VerseType, Song, Verse, Line, Size};
use crate::lexer::{Lexer};
use std::fmt;


pub struct ParseFormatter{
    song:Song
}
impl ParseFormatter{
    pub fn new(song: Song) -> ParseFormatter{
        ParseFormatter{
            song: song
        }
    }
    fn format_song_part(part: &SongPart) -> String{
        match part {
            SongPart::Text(t) => t.to_owned(),
            SongPart::Chord(t) => String::from(format!("<span class='chord'><strong class='chord'>{}</strong></span>", t.to_owned())),
            SongPart::Directive(DirectiveType::Comment(t)) => String::from(format!("<span class='comment'>{}</span>", t.to_owned())),
            _ => String::from("XXXXXX"),
        }
    }
    pub fn format(self) -> String{
        let mut output = String::new();
        output.push_str(&String::from(
                format!("\n<div class='song'>\n<p>sirka {}; vyska {};</p>\n\t<h1>{}</h1>\n", &self.song.width(), &self.song.height(), &self.song.title)));
        for ref verse in &self.song.verses{
         ////// V
            let chorus = match verse.verse_type{
                VerseType::Chorus => " chorus",
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
impl fmt::Display for ParseFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<h2>{}</h2>", &self.song.title);
        for verse in &self.song.verses{
            write!(f, "<div class='sloka'>{:?}</div>", &self.song.verses);
        }
        Ok(())
    }
}

use crate::model::{SongPart, DirectiveType, VerseType, Song, Verse, Line,};
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
    pub fn format(self) -> String{
        let mut output = String::new();
        output.push_str(&String::from(format!("\n<div class='song'>\n")));
        for ref verse in &self.song.verses{
            output.push_str(&String::from(format!("\t<div class='verse'>\n")));
            {
                for ref line in &verse.lines{
                    let mut has_chords = "";
                    if line.has_chords{
                        has_chords = " has_chords";
                    }
                    output.push_str(&String::from(format!("\t\t<p class='line{}'>{:?}</p>\n", has_chords, line)));
                }
            }
            output.push_str(&String::from(format!("\t</div>")));
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

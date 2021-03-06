use crate::model::{SongPart, DirectiveType, VerseType, Song};
use crate::Context;
use crate::Formatter;
use crate::FormatResult;

/// Example HTML formatter. Not considered to be used for anything useful.
pub struct ParseFormatter();

impl<T: Context> Formatter<T> for ParseFormatter{
    fn pre(&self, _context: &mut T) -> FormatResult{
    Ok("<html>
        <head><
        link rel='stylesheet' href='styl5.css'>
        <meta charset='utf-8'>
        </head>
        <body>".to_string())
    }

    fn format(&self, song: Song, context: &mut T) -> FormatResult{
        let mut output = String::new();
        let number = match context.get::<i32>("number"){
            Some(n) => *n,
            None => 0i32,
        } + 1;
        context.set("number", Box::new(number));
        output.push_str(&format!("\n<div class='song'>\n\t<h1><span class='number'>{}</span>{}</h1>\n", &number, &song.title));
        for ref verse in &song.verses{
            let chorus = match verse.verse_type{
                VerseType::Chorus => " ref",
                _ => "",
            };
            output.push_str(&format!("\t<div class='verse {}'>\n", chorus));
            {
                for ref line in &verse.lines{
                    let mut has_chords = "";
                    if line.has_chords{
                        has_chords = " has_chords";
                    }
                    output.push_str(&format!("\t\t<div class='line{}'>", has_chords));
                    for song_part in &line.song_parts{
                        output.push_str(self.format_song_part(song_part).as_str());
                    }
                    output.push_str("</div>\n");
                }
            }
            output.push_str("\t</div>\n");
        }
        output.push_str("</div>\n");
        Ok(output)
    }

    fn post(&self, _context: &mut T) -> FormatResult{
        Ok("</body></html>".to_string())
    }
}

impl ParseFormatter{

    fn format_song_part(&self, part: &SongPart) -> String{
        match part {
            SongPart::Text(t) => t.to_owned(),
            SongPart::Chord(t) => format!("<span class='chord'><strong class='chord'>{}</strong></span>", t.to_owned()),
            SongPart::Directive(DirectiveType::Comment(t)) => format!("<span class='comment'>{}</span>", t.to_owned()),
            _ => String::from(""),
        }
    }
}

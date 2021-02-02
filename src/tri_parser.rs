use crate::model::{SongPart, DirectiveType, VerseType, Song, Verse, Line,};
use crate::lexer::{Lexer};
use crate::Parser;

use std::str::Chars;

pub struct TriParser{
    one: Option<SongPart>,
    two: Option<SongPart>,
    three: Option<SongPart>,
}

impl TriParser {
    pub fn new() -> TriParser{
        TriParser{
            one: None,
            two: None,
            three: None,
        }
    }

}

impl Parser for TriParser {
    fn parse(&mut self, chars: Chars) -> Result<Song, String>{
        let mut lexer = Lexer::new(chars);    
        self.one = lexer.next();
        self.two = lexer.next();
        self.three = lexer.next();
        let mut song = Song{
            verses: vec![],
            title: "mock".to_string()
        };
        let verse_type = VerseType::Common;
        let mut verse = Verse{
            verse_type: verse_type,
            lines: vec![],
        };
        let mut line = Line{
            has_chords: false,
            song_parts: vec![],
        };

        let mut iter = ||{
            self.one = std::mem::replace(&mut self.two, std::mem::replace(&mut self.three, lexer.next()));
            if let None = self.three{
                self.three = Some(SongPart::Empty)
            }
            if let Some(SongPart::Empty) = self.two{
                self.two = Some(SongPart::Empty)
            }
            if let Some(SongPart::Empty) = self.one{
                return None
            }
            Some((self.one.clone(), self.two.clone(), self.three.clone()))
        };

        while let Some(triplet) = iter(){
            let triplet = unwrap(triplet);
            match triplet{
                // get title
                (SongPart::NewLine, SongPart::Directive(DirectiveType::Title(t)), _) => song.title = t,
                (SongPart::Directive(DirectiveType::Title(t)), _, _) => song.title = t,

                // end of song
                  //(NewLine, Text, None)
                  // (NewLine, Chord, None)
                  // (NewLine, Comment, None)

                (SongPart::Chord(ch), SongPart::Chord(ch2), SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Chord(ch));
                    line.song_parts.push(SongPart::Chord(ch2));
                    line.has_chords = true;
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },
                (SongPart::Chord(ch), SongPart::Text(t), SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Chord(ch));
                    line.song_parts.push(SongPart::Text(t));
                    line.has_chords = true;
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },

                (SongPart::Text(t), SongPart::Chord(ch2), SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Text(t));
                    line.song_parts.push(SongPart::Chord(ch2));
                    line.has_chords = true;
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },
                
                (SongPart::Chord(ch), SongPart::NewLine, SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Chord(ch));
                    line.has_chords = true;
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },

                (SongPart::Text(t), SongPart::NewLine, SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Text(t));
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },

                (SongPart::Text(t), SongPart::Text(t2), SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Text(t));
                    line.song_parts.push(SongPart::Text(t2));
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },

                (SongPart::Directive(DirectiveType::Comment(c)), SongPart::NewLine, SongPart::Empty) =>{
                    line.song_parts.push(SongPart::Directive(DirectiveType::Comment(c)));
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },

                (_, _, SongPart::Empty) =>{
                    verse.lines.push(line);
                    song.verses.push(verse);
                    break;
                },


                // start of verse TBD

                // end of verse
                (SongPart::Text(t),SongPart::NewLine, SongPart::NewLine) => {
                    line.song_parts.push(SongPart::Text(t));
                    verse.lines.push(line);
                    song.verses.push(verse);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    };
                    verse = Verse{
                        verse_type: VerseType::Common,
                        lines: vec![],
                    };
                },
                (SongPart::Chord(ch),SongPart::NewLine, SongPart::NewLine) =>{
                    line.song_parts.push(SongPart::Chord(ch));
                    line.has_chords = true;
                    verse.lines.push(line);
                    song.verses.push(verse);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    };
                    verse = Verse{
                        verse_type: VerseType::Common,
                        lines: vec![],
                    };
                },
                (SongPart::Directive(DirectiveType::Comment(c)),SongPart::NewLine, SongPart::NewLine) =>{
                    line.song_parts.push(SongPart::Directive(DirectiveType::Comment(c)));
                    verse.lines.push(line);
                    song.verses.push(verse);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    };
                    verse = Verse{
                        verse_type: VerseType::Common,
                        lines: vec![],
                    };
                },
                (SongPart::Directive(_),SongPart::NewLine, SongPart::NewLine) =>{
                    verse.lines.push(line);
                    song.verses.push(verse);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    };
                    verse = Verse{
                        verse_type: VerseType::Common,
                        lines: vec![],
                    };
                },

                // start of line
                (SongPart::NewLine, SongPart::Chord(_ch), _) => (),
                (SongPart::NewLine, SongPart::Text(_ch), _) => (),

                // end of line
                (SongPart::Text(t),SongPart::NewLine, _) => {
                    line.song_parts.push(SongPart::Text(t));
                    verse.lines.push(line);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    }
                },
                (SongPart::Chord(ch),SongPart::NewLine, _) =>{
                    line.song_parts.push(SongPart::Chord(ch));
                    line.has_chords = true;
                    verse.lines.push(line);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    }
                },
                    //also a comment; maybe chorus borders...

                (SongPart::Directive(DirectiveType::ChorusStart), SongPart::NewLine, SongPart::Text(_t)) =>{
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    };
                    //line.song_parts.push(SongPart::Text(t));
                    verse = Verse{
                        verse_type: VerseType::Chorus,
                        lines: vec![],
                    };
                }
                (SongPart::Directive(DirectiveType::ChorusStart), SongPart::NewLine, SongPart::Chord(ch)) =>{
                    verse = Verse{
                        verse_type: VerseType::Chorus,
                        lines: vec![],
                    };
                    line = Line{
                        has_chords:true,
                        song_parts: vec![],
                    };
                    line.song_parts.push(SongPart::Chord(ch));
                }


                // common
                (SongPart::Text(t), _, _) => line.song_parts.push(SongPart::Text(t)),
                (SongPart::Chord(ch), _, _) => {
                    line.song_parts.push(SongPart::Chord(ch));
                    line.has_chords = true;
                },
                _ => (),
            }
        }
        Ok(song)
    }
}

fn unwrap(triplet: (Option<SongPart>, Option<SongPart>, Option<SongPart>)) -> (SongPart, SongPart, SongPart){
    let (a, b, c) = triplet;
    (a.unwrap(), b.unwrap(), c.unwrap())
}



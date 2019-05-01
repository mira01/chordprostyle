use crate::model::{SongPart, DirectiveType, VerseType, Song, Verse, Line,};
use crate::lexer::{Lexer};

pub fn parse(lexer: Lexer) -> TriParser {
    TriParser::new(lexer)
}

pub struct TriParser<'a>{
    lexer: Lexer<'a>,
    one: Option<SongPart>,
    two: Option<SongPart>,
    three: Option<SongPart>,
}

impl<'a> TriParser<'a>{
    pub fn new(mut lexer: Lexer<'a>) -> TriParser<'a>{
        TriParser{
            one: lexer.next(),
            two: lexer.next(),
            three: lexer.next(),
            lexer: lexer,
        }
    }

    pub fn parse(&mut self) -> Song{
        let mut song = Song{
            verses: vec![],
            title: "mock".to_string()
        };
        let mut verse = Verse{
            verse_type: VerseType::Common,
            lines: vec![],
        };
        let mut line = Line{
            has_chords: false,
            song_parts: vec![],
        };

        for triplet in self{
            let triplet = unwrap(triplet);
            println!("{:?}", triplet);
            match triplet{
                // get title
                (SongPart::NewLine, SongPart::Directive(DirectiveType::Title(t)), _) => song.title = t,
                (SongPart::Directive(DirectiveType::Title(t)), _, _) => song.title = t,

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
                (SongPart::NewLine, SongPart::Chord(ch), _) => (),
                (SongPart::NewLine, SongPart::Text(ch), _) => (),

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
                    verse.lines.push(line);
                    line = Line{
                        has_chords:false,
                        song_parts: vec![],
                    }
                },
                    //also a comment; maybe chorus borders...

                // common
                (SongPart::Text(t), _, _) => line.song_parts.push(SongPart::Text(t)),
                (SongPart::Chord(ch), _, _) => line.song_parts.push(SongPart::Chord(ch)),
                _ => (),
            }
        }
        println!("line: {:?}", line);
        println!("verse: {:?}", verse);
        song
    }
}

fn unwrap(triplet: (Option<SongPart>, Option<SongPart>, Option<SongPart>)) -> (SongPart, SongPart, SongPart){
    let (a, b, c) = triplet;
    (a.unwrap(), b.unwrap(), c.unwrap())
}


impl<'a> Iterator for TriParser<'a>{
    type Item = (Option<SongPart>, Option<SongPart>, Option<SongPart>);

    fn next(&mut self) -> Option<Self::Item>{
        self.one = std::mem::replace(&mut self.two, std::mem::replace(&mut self.three, self.lexer.next()));
        if let None = self.three{
            return None
        }
        Some((self.one.clone(), self.two.clone(), self.three.clone()))
    }
}



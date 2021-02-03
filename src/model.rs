use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SongPart{
    Text(String),
    Chord(String),
    Directive(DirectiveType),
    NewLine,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType{
    Title(String),
    NewSong,
    ChorusStart,
    ChorusEnd,
    Comment(String),
    Other(String),
}

#[derive(Debug, PartialEq)]
pub struct Song{
    pub title: String,
    pub verses: Vec<Verse>,
}

impl Size for Song{
    fn width(&self) -> usize{
        self.verses.iter().map(|x|{x.width()}).max().unwrap()
    }
    fn height(&self) -> usize{
        let verses_height: usize = self.verses.iter().map(|x|{x.height()}).sum();
        let vertical_spaces: usize = self.verses.iter().count() -1;
        let title: usize = 3;
        verses_height + vertical_spaces + title
    }
}

#[derive(Debug, PartialEq)]
pub struct Verse{
    pub verse_type: VerseType,
    pub lines: Vec<Line>,
}
impl Size for Verse{
    fn width(&self) -> usize{
        self.lines.iter().map(|x|{x.width()}).max().unwrap()
    }
    fn height(&self) -> usize{
        self.lines.iter().map(|x|{x.height()}).sum()
    }
}

#[derive(Debug, PartialEq)]
pub enum VerseType{
    Common,
    Chorus,
}

#[derive(PartialEq)]
pub struct Line{
    pub has_chords: bool,
    pub song_parts: Vec<SongPart>,
}
impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for song_part in &self.song_parts{
            write!(f, "{:?}", song_part).unwrap();
        }
        Ok(())
    }
}
impl Size for Line{
    fn width(&self) -> usize{
        let len: usize = self.song_parts.iter().map(|x|{match x{
            SongPart::Text(t) => t.chars().count(),
            _ => 0
        }}).sum();
        len
    }
    fn height(&self) -> usize{
        if self.has_chords{
            2
        }else{
            1
        }
    }
}

pub trait Size{
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

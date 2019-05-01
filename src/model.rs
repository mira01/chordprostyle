use std::fmt;

#[derive(Debug, Clone)]
pub enum SongPart{
    Text(String),
    Chord(String),
    Directive(DirectiveType),
    NewLine,
    Empty,
}

#[derive(Debug, Clone)]
pub enum DirectiveType{
    Title(String),
    NewSong,
    ChorusStart,
    ChorusEnd,
    Comment(String),
    Other(String),
}

#[derive(Debug)]
pub struct Song{
    pub title: String,
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
pub struct Line{
    pub has_chords: bool,
    pub song_parts: Vec<SongPart>,
}
impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for song_part in &self.song_parts{
            write!(f, "{:?}", song_part);
        }
        Ok(())
    }
}

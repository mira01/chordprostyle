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
        output.push_str(&String::from(format!("\nSONG: ")));
        for verse in &self.song.verses{
            output.push_str(&String::from(format!("\n=>>{:?}", verse)));
        }
        output.push_str(&String::from(format!("\n\n<<=")));
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

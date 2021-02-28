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
    pub fn format(self) -> Result<String, LibError::FormatError>{
        let mut output = String::new();
        output.push_str(&String::from(format!("\n\n<div class='song'>")));
        for part in self.lexer{
            match part{
                SongPart::Text(text) => {
                    output.push_str(&format!("{}", &text));
                },
                SongPart::Directive(DirectiveType::Title(value))=> {
                    output.push_str(&format!("<h2>{}</h2>", &value));
                },
                SongPart::Directive(DirectiveType::ChorusStart)=> {
                    output.push_str(&format!("<div class='ref'>"));
                },
                SongPart::Directive(DirectiveType::ChorusEnd)=> {
                    output.push_str(&format!("</div>"));
                },
                SongPart::Directive(DirectiveType::Comment(value))=> {
                    output.push_str(&format!("<span class='comment'>{}</span>", value));
                },
                SongPart::Directive(value) => {
                    output.push_str(&String::from(format!("<div class='directive {:?}'>{:?}</div>", value, value)));
                },
                SongPart::Chord(text) => {
                    output.push_str(&format!("<span class='chord'><strong class='chord'>{}</span></strong>", text));
                },
                SongPart::NewLine =>{
                    output.push_str(&String::from("\n<br/>"));
                },
                _ => (),
            }
        }
        output.push_str(&String::from("</div>"));
        Ok(output)
    }
}


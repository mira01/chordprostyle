use tera::Tera;
use tera::Context as TeraContext;

use crate::model::{SongPart, DirectiveType, VerseType, Song};
use crate::Context;
use crate::Formatter;
use crate::FormatResult;
use crate::LibError;

pub struct TeraFormatter();

impl Formatter for TeraFormatter{
    fn pre(&self, _context: &mut Context) -> String{
    "<BEGIN>".to_string()
    }

    fn format(&self, song: Song, app_context: &mut Context) -> FormatResult{
        let mut tera = Tera::default();
        let mut context = TeraContext::new();
        context.insert("song", &song);
        let template = r#"
        <div class="song">
            <h1>{{ song.title }}</h1>
            {%- for verse in song.verses %}
               <div class="verse {{verse.verse_type}}">
                    {% for line in verse.lines -%}
                        <p class="line {%- if line.has_chords %} has_chords {%- endif %}">
                            {%- for part in line.song_parts -%}
                                {%- if part.type == "Chord" -%}
                                    <span class="chord"><strong>{{part.content}}</strong></span>
                                {%- else -%}
                                    {{part.content}}
                                {%- endif -%}
                            {%- endfor -%}
                        </p>
                    {% endfor %}
               </div>
            {%- endfor %}
        </div>
        "#;
        tera.add_raw_template("hello.html", template).expect("cannot add template");
        let res = tera.render("hello.html", &context).expect("cannot render");

        Ok(res.into())
    }

    fn post(&self, _context: &mut Context) -> String{
        "<END>".to_string()
    }
}

use tera::Tera;
use tera::Context as TeraContext;
use tera::Error as TeraError;

use crate::model::Song;
use crate::Context;
use crate::Formatter;
use crate::FormatResult;
use crate::LibError;

impl From<TeraError> for LibError{
    fn from(e: TeraError) -> LibError{
        LibError::FormatError(Box::new(e))
    }
}

/// Formatter based on Tera library.
///
/// Expects a Jinja2-like template described in [https://tera.netlify.app/docs/]
pub struct TeraFormatter<'a>{
    template: &'a str,
}

impl TeraFormatter<'_>{

    pub fn new(template: &str) -> TeraFormatter<'_>{
        TeraFormatter{ template }
    }

    fn render_template(&self, override_template: &str, tera_context: &TeraContext) -> FormatResult{
        let mut tera = Tera::default();
        tera.add_raw_template("base", &self.template)?;
        tera.add_raw_template("content", override_template)?;
        let res = tera.render("content", &tera_context)?;
        Ok(res)
    }
}

impl Formatter for TeraFormatter<'_>{

    fn pre(&self, _context: &mut Context) -> FormatResult{
        // The inheritance and overriding of parts in templates is here to comply with pre, format
        // and post methods in Formatter trait.
        // Basicaly I have three parts in base template and in each method of Formatter trait I
        // hide not wanted parts, keeping only the relevant one.
        let head_template = r#"{%- extends "base" -%}
        {%- block head -%}{{ super() }}{%- endblock head -%}
        {%- block footer -%}{%- endblock footer -%}
        {%- block songs -%}{%- endblock songs -%}"#;
        let context = TeraContext::new();
        self.render_template(head_template, &context)
    }

    fn format(&self, song: Song, _app_context: &mut Context) -> FormatResult{
        let songs_template = r#"{%- extends "base" -%}
        {%- block head -%}{%- endblock head -%}
        {%- block footer -%}{%- endblock footer -%}
        {%- block songs -%}{{ super() }}{%- endblock songs -%}"#;
        let mut context = TeraContext::new();
        context.insert("song", &song);
        self.render_template(songs_template, &context)
    }

    fn post(&self, _context: &mut Context) -> FormatResult{
        let footer_template = r#"{%- extends "base" -%}
        {%- block head -%}{%- endblock head -%}
        {%- block footer -%}{{ super() }}{%- endblock footer -%}
        {%- block songs -%}{%- endblock songs -%}"#;
        let context = TeraContext::new();
        self.render_template(footer_template, &context)
    }
}

impl Default for TeraFormatter<'_>{

    fn default() -> Self{
        const TEMPLATE: &str = r#"
            {% block head -%}
            <!DOCTYPE HTML>
            <html>
            <head></head>
            <body>
            {%- endblock head -%}
            {%- block songs -%}
            <div class="song">
                <h1>{{ song.title }}</h1>
                {%- for verse in song.verses %}
                   <div class="verse {{verse.verse_type}}">
                        {% for line in verse.lines -%}
                            <p class="line {%- if line.has_chords %} has_chords {%- endif %}">
                                {%- for part in line.song_parts -%}
                                    {%- if part.type == "Chord" -%}
                                        <span class="chord"><strong>{{part.content}}</strong></span>
                                    {%- elif part.type == "Directive" -%}
                                        <span class="{{part.content.type}}">{{part.content.content}}</span>
                                    {%- else -%}
                                        {{part.content}}
                                    {%- endif -%}
                                {%- endfor -%}
                            </p>
                        {% endfor %}
                   </div>
                {%- endfor %}
            </div>
            {%- endblock songs -%}
            {%- block footer -%}
            </body>
            </html>
            {%- endblock footer %}
            "#;
            Self::new(TEMPLATE)
    }
}

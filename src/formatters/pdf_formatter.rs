extern crate printpdf;
use printpdf::*;
use std::io::BufWriter;


pub struct PdfFormatter<'a>{
    lexer: Lexer<'a>
}
impl<'a> PdfFormatter<'a>{
    pub fn new(lexer: Lexer<'a>) -> PdfFormatter<'a>{
        PdfFormatter{lexer: lexer}
    }
    pub fn format(self) -> (){
        let doc_title = "Titulek";
        let (doc_width, doc_height) = (Mm(210.0), Mm(297.0));
        let (doc, page1, layer1) = PdfDocument::new(doc_title, doc_width, doc_height, "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let text = "Lorem ipsum";
        let text2 = "unicode: příliš žluťoučký kůň úpěl ďábelské ódy";

        let font2 = doc.add_external_font(File::open("/System/Library/Fonts/Palatino.ttc").unwrap()).unwrap();

        // text, font size, x from left edge, y from top edge, font
        ////current_layer.use_text(text, 14, Mm(20.0), Mm(20.0), &font2);

        // For more complex layout of text, you can use functions
        // defined on the PdfLayerReference
        // Make sure to wrap your commands
        // in a `begin_text_section()` and `end_text_section()` wrapper
        current_layer.begin_text_section();

            // setup the general fonts.
            // see the docs for these functions for details
            current_layer.set_font(&font2, 14);
            current_layer.set_text_cursor(Mm(10.0), Mm(270.0));
            current_layer.set_line_height(33);
            current_layer.set_word_spacing(3000);
            current_layer.set_character_spacing(10);
            //current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

            // write two lines (one line break)
            current_layer.write_text(text.clone(), &font2);
            current_layer.add_line_break();
            current_layer.write_text(text2.clone(), &font2);
            current_layer.add_line_break();

            // write one line, but write text2 in superscript
            current_layer.write_text(text.clone(), &font2);
            current_layer.set_line_offset(10);
            current_layer.write_text(text2.clone(), &font2);

    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
    //for a in result{
    //    println!("{:?}", a);
    //}

    }
}


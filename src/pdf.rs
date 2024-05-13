use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::chart::Chart;

const DOC_HEIGHT: f32 = 297.0;
const DOC_WIDTH: f32 = 210.0;
const FONT_SIZE_SUBTITLE: f32 = 10.0;
const FONT_SIZE_BODY: f32 = 14.0;
const FONT_SIZE_TITLE: f32 = 24.0;
const PADDING_MM: f32 = 4.233;
const FONT_TITLE_MM: f32 = FONT_SIZE_TITLE / 3.7795;
const FONT_BODY_MM: f32 = FONT_SIZE_BODY / 3.7795;

pub fn create_pdf(chart: Chart) {
    let (doc, page1, layer1) = PdfDocument::new(
        "PDF_Document_title",
        Mm(DOC_WIDTH),
        Mm(DOC_HEIGHT),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .expect("pdf: Font not loaded.");

    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .expect("pdf: Font not loaded.");

    let font_italic = doc
        .add_builtin_font(BuiltinFont::HelveticaOblique)
        .expect("pdf: Font not loaded.");

    current_layer.begin_text_section();

    current_layer.set_font(&font_bold, FONT_SIZE_TITLE);
    current_layer.set_line_height(FONT_SIZE_BODY * 1.2);
    current_layer.set_text_cursor(
        Mm(PADDING_MM * 2.0),
        Mm(DOC_HEIGHT - FONT_TITLE_MM - { PADDING_MM * 2.0 }),
    );

    if let Some(title) = &chart.title {
        current_layer.write_text(title, &font_bold);
        current_layer.add_line_break();
    }

    if let Some(artist) = &chart.artist {
        current_layer.set_font(&font_italic, FONT_SIZE_SUBTITLE);
        current_layer.write_text(String::from("Artist: ") + artist, &font_italic);
        current_layer.add_line_break();
    }

    if let Some(tempo) = &chart.tempo {
        current_layer.set_font(&font_italic, FONT_SIZE_SUBTITLE);
        current_layer.write_text(
            String::from("Tempo: ") + tempo + &String::from("  |  "),
            &font_italic,
        );
    }

    if let Some(key) = &chart.key {
        current_layer.set_font(&font_italic, FONT_SIZE_SUBTITLE);
        current_layer.write_text(String::from("Key: ") + key, &font_italic);
    }

    current_layer.add_line_break();
    current_layer.add_line_break();

    chart.sections.iter().for_each(|s| {
        current_layer.set_font(&font_bold, FONT_SIZE_BODY);
        current_layer.write_text(&s.title, &font_bold);
        current_layer.add_line_break();

        current_layer.set_font(&font, FONT_SIZE_BODY);
        s.lines.iter().for_each(|l| {
            l.iter().for_each(|p| {
                if let Some(pair) = &p {
                    current_layer.write_text(&pair.lyric, &font);
                }
            });
            current_layer.add_line_break();
        });
        current_layer.add_line_break();
    });
    current_layer.end_text_section();

    let file_name: String;
    match (chart.title, chart.key) {
        (Some(title), Some(key)) => {
            file_name =
                title.split(" ").collect::<Vec<&str>>().join("-") + &String::from("-") + &key
        }
        (Some(title), None) => file_name = title,
        _ => file_name = String::from("new chart"),
    }

    doc.save(&mut BufWriter::new(
        File::create(file_name + ".pdf").expect("Could not create pdf."),
    ))
    .expect("Could not save file");
}

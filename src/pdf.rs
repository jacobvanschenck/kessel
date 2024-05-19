use printpdf::*;
use rusttype::{point, Font, HMetrics, Scale};
use std::fs::File;
use std::io::BufWriter;

use crate::chart::Chart;

const DOC_HEIGHT: f32 = 297.0;
const DOC_WIDTH: f32 = 210.0;
const FONT_SIZE_SUBTITLE: f32 = 10.0;
const FONT_SIZE_BODY: f32 = 14.0;
const FONT_SIZE_TITLE: f32 = 24.0;
const PADDING_MM: f32 = 4.233;
// const FONT_TITLE_MM: f32 = FONT_SIZE_TITLE / 3.7795;
// const FONT_BODY_MM: f32 = FONT_SIZE_BODY / 3.7795;

pub fn create_pdf(chart: Chart) {
    let (doc, page1, layer1) = PdfDocument::new(
        "PDF_Document_title",
        Mm(DOC_WIDTH),
        Mm(DOC_HEIGHT),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font_data = include_bytes!("Roboto-Regular.ttf");
    let font_bold_data = include_bytes!("Roboto-Bold.ttf");
    let font_italic_data = include_bytes!("Roboto-Italic.ttf");

    let (font, font_bold, font_italic) = load_fonts(&doc);

    current_layer.begin_text_section();

    let mut current_x = PADDING_MM * 2.0;
    let mut current_y = DOC_HEIGHT - { FONT_SIZE_TITLE / 3.7795 } - { PADDING_MM * 2.0 };

    current_layer.set_line_height(FONT_SIZE_BODY * 1.2);

    if let Some(title) = &chart.title {
        current_layer.use_text(
            title,
            FONT_SIZE_TITLE,
            Mm(current_x),
            Mm(current_y),
            &font_bold,
        );
        let (_, height) = get_text_width(font_bold_data, FONT_SIZE_TITLE / 3.7795, title);
        current_y -= height;
    }

    if let Some(artist) = &chart.artist {
        let str = String::from("Artist: ") + artist;
        current_layer.use_text(
            &str,
            FONT_SIZE_SUBTITLE,
            Mm(current_x),
            Mm(current_y),
            &font_italic,
        );
        let (_, height) = get_text_width(font_italic_data, FONT_SIZE_SUBTITLE / 3.7795, &str);
        current_y -= height * 1.4;
    }

    if let Some(tempo) = &chart.tempo {
        let str = String::from("Tempo: ") + tempo + " |";
        current_layer.use_text(
            &str,
            FONT_SIZE_SUBTITLE,
            Mm(current_x),
            Mm(current_y),
            &font_italic,
        );
        let (width, _) = get_text_width(font_italic_data, FONT_SIZE_SUBTITLE / 3.7795, &str);
        current_x += width;
    }

    if let Some(key) = &chart.key {
        let str = String::from(" Key: ") + key;
        current_layer.use_text(
            &str,
            FONT_SIZE_SUBTITLE,
            Mm(current_x),
            Mm(current_y),
            &font_italic,
        );
    }

    let (_, height) = get_text_width(font_italic_data, FONT_SIZE_SUBTITLE / 3.7795, "hoo");

    current_x = PADDING_MM * 2.0;
    current_y -= height * 2.0 * 1.4;

    chart.sections.iter().for_each(|s| {
        current_layer.use_text(
            &s.title,
            FONT_SIZE_BODY,
            Mm(current_x),
            Mm(current_y),
            &font_bold,
        );

        let (_, height) = get_text_width(font_data, FONT_SIZE_BODY / 3.7795, &s.title);
        current_y -= height * 1.4;

        current_layer.set_font(&font, FONT_SIZE_BODY);
        s.lines.iter().for_each(|l| {
            l.iter().for_each(|p| {
                if let Some(pair) = &p {
                    if let Some(chord) = &pair.chord {
                        current_layer.use_text(
                            &*chord,
                            FONT_SIZE_BODY,
                            Mm(current_x),
                            Mm(current_y),
                            &font,
                        );
                    };
                    current_layer.use_text(
                        &pair.lyric,
                        FONT_SIZE_BODY,
                        Mm(current_x),
                        Mm(current_y - { height * 1.4 }),
                        &font,
                    );
                    let (width, _) =
                        get_text_width(font_data, FONT_SIZE_BODY / 3.7795, &pair.lyric);
                    current_x += width;
                }
            });
            current_y -= height * 1.4 * 2.0;
            current_x = PADDING_MM * 2.0;
        });
        current_y -= height * 1.4;
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

fn get_text_width(font_data: &[u8], size: f32, text: &str) -> (f32, f32) {
    println!("string: {}", text);
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(size);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as f32
    };

    (glyphs_width, glyphs_height)
}

fn load_fonts(doc: &PdfDocumentReference) -> (IndirectFontRef, IndirectFontRef, IndirectFontRef) {
    let font_data = include_bytes!("Roboto-Regular.ttf");
    let mut font_reader = std::io::Cursor::new(font_data.as_ref());
    let font = doc
        .add_external_font(&mut font_reader)
        .expect("pdf: Font not loaded.");

    let font_data = include_bytes!("Roboto-Bold.ttf");
    font_reader = std::io::Cursor::new(font_data.as_ref());
    let font_bold = doc
        .add_external_font(&mut font_reader)
        .expect("pdf: Font Bold not loaded.");

    let font_data = include_bytes!("Roboto-Italic.ttf");
    font_reader = std::io::Cursor::new(font_data.as_ref());
    let font_italic = doc
        .add_external_font(&mut font_reader)
        .expect("pdf: Font Italic not loaded.");

    (font, font_bold, font_italic)
}

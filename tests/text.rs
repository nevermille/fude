use fude::pdf::font::{Font, FontSubtype};
use fude::traits::IdentifiedObject;
use fude::types::PageFormat;
use fude::Fude;

#[test]
fn hello_world() {
    let mut fude = Fude::new();
    let mut font = Font::from_document(&mut fude.inner_doc);

    fude.add_new_page(&PageFormat::A4Portrait);
    font.file = "assets/fonts/OpenSans-Regular.ttf".to_string();
    font.subtype = FontSubtype::TrueType;
    font.font_descriptor
        .read_font("assets/fonts/OpenSans-Regular.ttf");
    font.associated_text = "Hello world!".to_string();

    fude.central_library.font.insert("F1".to_string(), font);
    fude.gen("/Users/camillebernard/Desktop/hello_world.pdf");
}

use fude::builders::FontBuilder;
use fude::pdf::font::{Font, FontSubtype};
use fude::traits::IdentifiedObject;
use fude::types::PageFormat;
use fude::Fude;

#[test]
fn hello_world() {
    let mut fude = Fude::new();
    let mut open_sans_regular = FontBuilder::from_path("assets/fonts/OpenSans-Regular.ttf");
    let open_sans_regular = fude.font_add(open_sans_regular);

    assert_eq!(open_sans_regular, "F0");

    fude.add_new_page(&PageFormat::A4Portrait);
    let f = fude.central_library.font.get_mut("F0").unwrap();
    f.associated_text = "hello world!".to_string();

    fude.gen("target/tmp/hello_world.pdf");
}

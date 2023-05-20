use fude::types::PageFormat;
use fude::Fude;

#[test]
fn create_one_page_blank_document() {
    let mut fude = Fude::new();

    assert_eq!(fude.add_new_page(&PageFormat::A4Portrait), 1);
    assert!(fude.gen("target/tmp/blank_test.pdf"));
}

#[test]
fn create_multiple_pages_blank_document() {
    let mut fude = Fude::new();

    assert_eq!(fude.add_new_page(&PageFormat::A4Portrait), 1);
    assert_eq!(fude.add_new_page(&PageFormat::A4Landscape), 2);
    assert!(fude.gen("target/tmp/blank_multiple_test.pdf"));
}

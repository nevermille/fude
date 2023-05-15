use mudpf::mupdf::MuPdf;
use mudpf::types::PageFormat;

#[test]
fn create_one_page_blank_document() {
    let mut mupdf = MuPdf::new();

    assert_eq!(mupdf.add_new_page(&PageFormat::A4Portrait), 1);
    assert!(mupdf.gen("/Users/camillebernard/Desktop/blank_test.pdf"));
}

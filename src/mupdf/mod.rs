mod finalize;
mod font;
mod text;

use crate::{font::ttf::TtfFont, unit::PageFormat, Page};
use lopdf::{dictionary, Dictionary, Document, Object, ObjectId};
use std::{fs::File, io::Error};

use self::font::FontId;

/// The main class managing the PDF document
pub struct MuPDF {
    /// The document in writing
    inner_doc: Document,
    /// The root pages object id
    pages_id: ObjectId,
    /// The pages' list
    pages: Vec<Page>,
    ttf_fonts: Vec<TtfFont>,
    brush_position: (i64, i64),
    brush_font: FontId,
    resources_id: ObjectId,
    resources: Dictionary,
    default_page_format: PageFormat,
    /// An external document opened
    outer_doc: Option<Document>,
}

impl MuPDF {
    /// Creates a new document (version 1.7)
    pub fn new() -> Self {
        let mut inner_doc = Document::with_version("1.7");
        let pages_id = inner_doc.new_object_id();
        let resources_id = inner_doc.new_object_id();

        Self {
            inner_doc,
            pages_id,
            outer_doc: None,
            pages: Vec::new(),
            default_page_format: PageFormat::A4P,
            resources_id,
            ttf_fonts: Vec::new(),
            brush_position: (0, 0),
            brush_font: FontId::Ttf(0),
            resources: Dictionary::new(),
        }
    }

    pub fn add_page(&mut self) {
        let page_id = self.inner_doc.new_object_id();
        let content_id = self.inner_doc.new_object_id();
        let page = Page::from_format(self.pages_id, content_id, page_id, None);

        self.pages.push(page);
    }

    pub fn save(&mut self, path: &str) -> Result<File, Error> {
        self.inner_doc.save(path)
    }
}

impl Default for MuPDF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::MuPDF;

    #[test]
    pub fn blank_pdf() {
        let mut pdf = MuPDF::new();
        pdf.add_page();
        pdf.print_text("Hello World!");
        pdf.finalize();
        pdf.save("/tmp/mupdf.pdf").unwrap();
    }
}

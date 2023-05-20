use crate::pdf::font::Font;
use crate::traits::IdentifiedObject;
use lopdf::Document;

pub struct FontBuilder {
    pub path: String,
}

impl FontBuilder {
    /// Creates a new builder
    pub fn from_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn build(self, document: &mut Document) -> Font {
        let mut font = Font::from_document(document);

        font.file = self.path.clone();
        font.font_descriptor.read_font(&self.path);

        font
    }
}

use crate::{font::ttf::TtfFont, MuPDF};
use std::error::Error;

pub enum FontId {
    Ttf(usize),
}

impl MuPDF {
    pub fn add_ttf_font(&mut self, file: &str) -> Result<FontId, Box<dyn Error>> {
        let font = TtfFont::from_file(
            file,
            (
                self.inner_doc.new_object_id(),
                self.inner_doc.new_object_id(),
                self.inner_doc.new_object_id(),
                self.inner_doc.new_object_id(),
            ),
        )?;
        self.ttf_fonts.push(font);

        Ok(FontId::Ttf(self.ttf_fonts.len() - 1))
    }
}

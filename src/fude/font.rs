use crate::builders::FontBuilder;
use crate::Fude;

impl Fude {
    /// Add a font to the document
    ///
    /// # Parameters
    ///
    /// * `builder`: A font builder
    pub fn font_add(&mut self, builder: FontBuilder) -> String {
        let font_id = format!("F{}", self.central_library.font.len());
        let font = builder.build(&mut self.inner_doc);

        self.central_library.font.insert(font_id.clone(), font);

        font_id
    }
}

use crate::mupdf::MuPdf;
use crate::traits::{IdentifiedObject, PdfIntegrate};

impl MuPdf {
    /// Generates the document
    pub fn gen(&self, path: &str) -> bool {
        let mut out_doc = self.inner_doc.clone();

        // Integrate all objects into document
        self.root.integrate_into_document(&mut out_doc);

        // Create trailer
        out_doc.trailer.set("Root", self.root.to_object_id());

        out_doc.save(path).is_ok()
    }
}

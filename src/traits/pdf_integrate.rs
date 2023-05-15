use crate::traits::{IdentifiedObject, ObjectExport};
use lopdf::Document;

/// Makes the struct able to integrate a document
pub trait PdfIntegrate: IdentifiedObject + ObjectExport {
    /// Integrate the struct into the document
    ///
    /// If the struct have childs like a Pages struct, you shall integrate them too
    fn integrate_into_document(&self, document: &mut Document);
}

use lopdf::{Document, ObjectId};

/// Adds functions to identify this object into the PDF
pub trait IdentifiedObject {
    /// Creates a new object with an auto generated id
    fn from_document(document: &mut Document) -> Self;

    /// Exports the struct into a PDF object id
    fn to_object_id(&self) -> ObjectId;
}

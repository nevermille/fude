use lopdf::{Object, ObjectId};

/// Makes the structure able to convert into an object
pub trait ObjectExport {
    /// Exports the struct into a PDF object
    fn to_object(&self) -> Object;
}

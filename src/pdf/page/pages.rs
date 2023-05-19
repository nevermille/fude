use crate::macros::log::info;
use crate::pdf::page::Page;
use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId};

/// The pages of a document are accessed through a structure known as
/// the page tree, which defines the ordering of pages in the document
pub struct Pages {
    /// The object id
    pub id: ObjectId,

    /// The number of leaf nodes (page objects) that are descendants
    /// of this node within the page tree
    pub count: i64,

    /// An array of indirect references to the immediate children of this node
    pub kids: Vec<Page>,
}

impl ObjectExport for Pages {
    fn to_object(&self) -> Object {
        Dictionary(dictionary! {
            "Type" => "Pages",
            "Count" => Object::Integer(self.count),
            "Kids" => self.kids.to_object()
        })
    }
}

impl PdfIntegrate for Pages {
    fn integrate_into_document(&self, document: &mut Document) {
        info!("Document <- Pages ({} {} R)", self.id.0, self.id.1);
        document.objects.insert(self.id, self.to_object());

        for page in &self.kids {
            page.integrate_into_document(document);
        }
    }
}

impl IdentifiedObject for Pages {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
            count: 0,
            kids: Vec::new(),
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

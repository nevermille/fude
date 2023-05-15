use crate::pdf::page::Pages;
use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId};

/// The root of a document’s object hierarchy is the catalog dictionary,
/// located by means of the Root entry in the trailer of the PDF file
pub struct Root {
    /// The object's id
    pub id: ObjectId,

    /// The page tree node that shall be the root of the document’s page tree
    pub pages: Pages,
}

impl IdentifiedObject for Root {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
            pages: Pages::from_document(document),
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

impl ObjectExport for Root {
    fn to_object(&self) -> Object {
        Dictionary(dictionary! {
            "Type" => "Catalog",
            "Pages" => Object::Reference(self.pages.to_object_id())
        })
    }
}

impl PdfIntegrate for Root {
    fn integrate_into_document(&self, document: &mut Document) {
        document.objects.insert(self.id, self.to_object());
        self.pages.integrate_into_document(document);
    }
}

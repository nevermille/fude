use crate::macros::log::info;
use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use crate::types::PageFormat;
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId};

/// The leaves of the page tree are page objects, each of which is a
/// dictionary specifying the attributes of a single page of the document
pub struct Page {
    /// The object id
    pub id: ObjectId,

    /// The page tree node that is the immediate parent of this page object
    pub parent: ObjectId,

    /// A dictionary containing any resources required by the page contents
    pub resources: ObjectId,

    /// A rectangle, expressed in default user space units, that shall define
    /// the boundaries of the physical medium on which the page shall be displayed or printed
    pub media_box: PageFormat,
}

impl IdentifiedObject for Page {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
            parent: (0, 0),
            resources: (0, 0),
            media_box: PageFormat::A4Portrait,
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

impl ObjectExport for Page {
    fn to_object(&self) -> Object {
        Dictionary(dictionary! {
            "Type" => "Page",
            "Parent" => Object::Reference(self.parent),
            "Resources" => Object::Reference(self.resources),
            "MediaBox" => self.media_box.to_object(),
        })
    }
}

impl ObjectExport for Vec<Page> {
    fn to_object(&self) -> Object {
        let mut object: Vec<Object> = Vec::new();

        // Get all references
        for page in self {
            object.push(Object::Reference(page.to_object_id()));
        }

        Object::Array(object)
    }
}

impl PdfIntegrate for Page {
    fn integrate_into_document(&self, document: &mut Document) {
        info!("Document <- Page ({} {} R)", self.id.0, self.id.1);
        document.objects.insert(self.id, self.to_object());
    }
}

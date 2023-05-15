use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId};

pub struct Resources {
    pub id: ObjectId,
}

impl IdentifiedObject for Resources {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

impl ObjectExport for Resources {
    fn to_object(&self) -> Object {
        Dictionary(dictionary! {})
    }
}

impl PdfIntegrate for Resources {
    fn integrate_into_document(&self, document: &mut Document) {
        document.objects.insert(self.id, self.to_object());
    }
}

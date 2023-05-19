use crate::pdf::font::Font;
use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use lopdf::Dictionary as PdfDictionary;
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId};
use std::collections::HashMap;

/// Resources for pages
pub struct Resources {
    /// The object's id
    pub id: ObjectId,

    /// The font list
    pub font: HashMap<String, Font>,
}

impl IdentifiedObject for Resources {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
            font: HashMap::new(),
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

impl ObjectExport for Resources {
    fn to_object(&self) -> Object {
        let mut font_dictionary = PdfDictionary::new();

        for (font_code, font) in &self.font {
            font_dictionary.set(font_code.as_bytes(), font.to_object_id());
        }

        Dictionary(dictionary! {
            "Font" => Dictionary(font_dictionary)
        })
    }
}

impl PdfIntegrate for Resources {
    fn integrate_into_document(&self, document: &mut Document) {
        document.objects.insert(self.id, self.to_object());

        for font in self.font.values() {
            font.integrate_into_document(document);
        }
    }
}

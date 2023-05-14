use lopdf::{dictionary, Dictionary, Object};

use crate::MuPDF;

impl MuPDF {
    pub fn finalize_pages(&mut self) {
        let mut kids: Vec<Object> = Vec::new();

        for page in &self.pages {
            self.inner_doc.objects.insert(
                page.id,
                Object::Dictionary(dictionary! {
                    "Type" => "Page",
                    "Parent" => Object::Reference(self.pages_id),
                    "Contents" => Object::Reference(page.content_id)
                }),
            );

            kids.push(page.to_object());
        }

        let pages = dictionary! {
            "Type" => "Pages",
            "Count" => Object::Integer(kids.len().try_into().unwrap_or_default()),
            "Kids" => Object::Array(kids),
            "Ressources" => self.resources_id,
            "MediaBox" => Object::Array(self.default_page_format.to_vec())
        };

        self.inner_doc
            .objects
            .insert(self.pages_id, Object::Dictionary(pages));
    }

    pub fn finalize_fonts(&mut self) {
        let mut fonts_dictionary = Dictionary::new();

        for font in &self.ttf_fonts {
            fonts_dictionary.set(format!("T0"), font.to_object());
        }

        self.resources.set("Fonts", fonts_dictionary);
    }

    pub fn finalize(&mut self) {
        self.finalize_fonts();
        self.finalize_pages();

        self.inner_doc.objects.insert(
            self.resources_id,
            Object::Dictionary(self.resources.clone()),
        );

        let catalog_id = self.inner_doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => self.pages_id
        });

        self.inner_doc.trailer.set("Root", catalog_id);
        self.inner_doc.compress();
    }
}

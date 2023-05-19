mod font;
mod gen;

use crate::pdf::page::Page;
use crate::pdf::resource::Resources;
use crate::pdf::root::Root;
use crate::traits::IdentifiedObject;
use crate::types::PageFormat;
use lopdf::Document;

/// The Fude manager
pub struct Fude {
    /// The document in creation
    pub inner_doc: Document,

    /// The document root
    pub root: Root,

    /// The central resources for the document
    pub central_library: Resources,
}

impl Fude {
    /// Creates a blank default document
    pub fn new() -> Self {
        let mut inner_doc = Document::with_version("2.0");
        let root = Root::from_document(&mut inner_doc);
        let central_library = Resources::from_document(&mut inner_doc);

        Self {
            inner_doc,
            root,
            central_library,
        }
    }

    /// Adds a new blank page and returns its number
    ///
    /// # Parameters
    ///
    /// * `format`: The page format
    pub fn add_new_page(&mut self, format: &PageFormat) -> i64 {
        // Create page
        let page = Page {
            parent: self.root.pages.to_object_id(),
            media_box: format.clone(),
            resources: self.central_library.to_object_id(),
            ..Page::from_document(&mut self.inner_doc)
        };

        // Add page and update count
        self.root.pages.kids.push(page);
        self.root.pages.count += 1;

        // The page number conveniently is the page count
        self.root.pages.count
    }
}

impl Default for Fude {
    fn default() -> Self {
        Self::new()
    }
}

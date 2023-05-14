use lopdf::{
    content::{Content, Operation},
    dictionary, Error, Object, ObjectId, Stream,
};

use crate::unit::PageFormat;

#[derive(Clone)]
pub struct Page {
    operations: Vec<Operation>,
    format: Option<PageFormat>,
    pub parent_id: ObjectId,
    pub content_id: ObjectId,
    pub id: ObjectId,
}

impl Page {
    pub fn from_format(
        parent_id: ObjectId,
        content_id: ObjectId,
        id: ObjectId,
        format: Option<PageFormat>,
    ) -> Self {
        Self {
            operations: Vec::new(),
            format,
            parent_id,
            content_id,
            id,
        }
    }

    pub fn add_operations(&mut self, operations: &mut Vec<Operation>) {
        self.operations.append(operations);
    }

    pub fn to_stream(&self) -> Result<Stream, Error> {
        let content = Content {
            operations: self.operations.clone(),
        };

        Ok(Stream::new(dictionary! {}, content.encode()?))
    }

    pub fn to_object(&self) -> Object {
        Object::Reference(self.id)
    }
}

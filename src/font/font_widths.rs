use lopdf::{Object, ObjectId};

pub struct FontWidths {
    widths: Vec<Object>,
    id: ObjectId,
}

impl FontWidths {
    pub fn new(id: ObjectId) -> Self {
        Self {
            widths: Vec::new(),
            id,
        }
    }

    pub fn add_width(&mut self, width: i64) {
        self.widths.push(Object::Integer(width));
    }

    pub fn to_object(&self) -> Object {
        Object::Array(self.widths.clone())
    }
}

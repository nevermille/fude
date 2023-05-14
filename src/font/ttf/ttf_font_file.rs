use lopdf::{dictionary, Object, ObjectId, Stream};

pub struct TtfFontFile {
    bin: Vec<u8>,
    id: ObjectId,
}

impl TtfFontFile {
    pub fn from_bytes(bin: Vec<u8>, id: ObjectId) -> Self {
        Self { bin, id }
    }

    pub fn to_stream(&self) -> Stream {
        Stream::new(
            dictionary! {"Lenght1" => Object::Integer(self.bin.len() as i64) },
            self.bin.clone(),
        )
    }

    pub fn to_object(&self) -> Object {
        Object::Reference(self.id)
    }
}

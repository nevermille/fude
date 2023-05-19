use crate::traits::ObjectExport;
use lopdf::{Object, StringFormat};

pub enum FontSubtype {
    Type1,
    TrueType,
    Type3,
}

impl ObjectExport for FontSubtype {
    fn to_object(&self) -> Object {
        match self {
            Self::TrueType => Object::String(b"TrueType".to_vec(), StringFormat::Literal),
            Self::Type1 => Object::String(b"Type1".to_vec(), StringFormat::Literal),
            Self::Type3 => Object::String(b"Type3".to_vec(), StringFormat::Literal),
        }
    }
}

use crate::traits::ObjectExport;
use lopdf::{Object, StringFormat};

/// A name specifying the format of the embedded font program
pub enum FontSubsubtype {
    /// Type 1 compact fonts
    Type1C,

    /// Type 0 compact CIDFonts
    CIDFontType0C,

    /// OpenType fonts
    OpenType,
}

impl ObjectExport for FontSubsubtype {
    fn to_object(&self) -> Object {
        match self {
            Self::Type1C => Object::String(b"Type1C".to_vec(), StringFormat::Literal),
            Self::CIDFontType0C => Object::String(b"CIDFontType0C".to_vec(), StringFormat::Literal),
            Self::OpenType => Object::String(b"OpenType".to_vec(), StringFormat::Literal),
        }
    }
}

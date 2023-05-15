use crate::traits::ObjectExport;
use crate::types::distance::Distance;
use lopdf::{Object, ObjectId};

/// A page format
#[derive(Clone)]
pub enum PageFormat {
    /// ISO A4 portrait
    A4Portrait,

    /// ISO A4 landscape
    A4Landscape,

    /// Custom format
    Custom(Distance, Distance),
}

impl ObjectExport for PageFormat {
    fn to_object(&self) -> Object {
        match self {
            PageFormat::A4Portrait => Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                Distance::Mm(210.0).to_object(),
                Distance::Mm(297.0).to_object(),
            ]),
            PageFormat::A4Landscape => Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                Distance::Mm(297.0).to_object(),
                Distance::Mm(210.0).to_object(),
            ]),
            PageFormat::Custom(w, h) => Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                w.to_object(),
                h.to_object(),
            ]),
        }
    }
}

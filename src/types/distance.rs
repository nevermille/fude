use crate::traits::ObjectExport;
use lopdf::Object;

/// A measure of a distance
#[derive(Clone)]
pub enum Distance {
    /// Centimeters
    Cm(f32),

    /// Millimeters
    Mm(f32),

    /// Inches
    In(f32),

    /// Points (PDF internal unit)
    Pt(f32),
}

impl Distance {
    /// Converts the distance unit to points
    pub fn to_pt(&self) -> f32 {
        match self {
            Self::Cm(v) => v / 28.329,
            Self::Mm(v) => v * 2.833,
            Self::In(v) => v * 72.0,
            Self::Pt(v) => *v,
        }
    }
}

impl ObjectExport for Distance {
    fn to_object(&self) -> Object {
        Object::Real(self.to_pt())
    }
}

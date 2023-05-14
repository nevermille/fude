use lopdf::Object;

pub enum Size {
    Mm(f64),
    Cm(f64),
    In(f64),
    Pt(f64),
}

#[derive(Clone)]
pub enum PageFormat {
    A4P,
    A4L,
}

impl PageFormat {
    pub fn to_vec(&self) -> Vec<Object> {
        match self {
            Self::A4P => vec![0.into(), 0.into(), 595.into(), 842.into()],
            Self::A4L => vec![0.into(), 0.into(), 842.into(), 595.into()],
        }
    }
}

pub type Coordinate = Size;

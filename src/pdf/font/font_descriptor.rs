use crate::macros::log::{error, info, warning};
use crate::pdf::font::font_stretch::FontStretch;
use crate::pdf::font::{FontSubsubtype, FontSubtype, FontWeight};
use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId, StringFormat};
use ttf_parser::name::Table as NameTable;
use ttf_parser::name_id::{FAMILY, FULL_NAME};
use ttf_parser::Face;

pub struct FontDescriptor {
    pub id: ObjectId,
    pub font_name: String,
    pub font_family: String,
    pub flags: i64,
    pub font_bbox: Vec<f32>,
    pub italic_angle: f32,
    pub ascent: f32,
    pub descent: f32,
    pub cap_height: f32,
    pub stem_v: f32,
    pub font_weight: FontWeight,
    pub stretch: FontStretch,
    pub subtype: FontSubtype,
    pub subsubtype: Option<FontSubsubtype>,
    pub font_file: ObjectId,
}

impl FontDescriptor {
    /// Reads font names
    ///
    /// # Parameters
    ///
    /// * `name_table`: The names ttf table
    fn read_names(&mut self, name_table: Option<NameTable>) {
        if name_table.is_none() {
            warning!("Unable to find name table");
            return;
        }

        // Read names
        // We need to remove \0 characters, don't ask me why they are here
        for name in name_table.as_ref().unwrap().names {
            match name.name_id {
                FULL_NAME => self.font_name = String::from_utf8_lossy(name.name).replace('\0', ""),
                FAMILY => self.font_family = String::from_utf8_lossy(name.name).replace('\0', ""),
                _ => {}
            }
        }
    }

    /// Reads the font and populates info
    ///
    /// # Parameters
    ///
    /// * `font_path`: The font file's path
    pub fn read_font(&mut self, font_path: &str) {
        info!("Try to read font");

        // Extract font bytes
        let raw_data = std::fs::read(font_path);
        if let Err(e) = raw_data {
            error!("Unable to read font file: {}", e);
            return;
        }
        let raw_data = raw_data.unwrap();

        info!("Font read, try to parse font");

        // Parse font
        let font_face = Face::parse(&raw_data, 0);
        if let Err(e) = font_face {
            error!("Unable to parse font file: {}", e);
            return;
        }
        let font_face = font_face.unwrap();

        info!("Font parsed, try to extract info");

        // Extract tables
        let font_tables = font_face.tables();
        let font_head = font_tables.head;
        let font_hhea = font_tables.hhea;
        let font_name = font_tables.name;
        let font_post = font_tables.post;
        let font_os2 = font_tables.os2;

        // Extract values
        self.ascent = font_hhea.ascender as f32;
        self.descent = font_hhea.descender as f32;
        self.read_names(font_name);
        self.font_weight = FontWeight::from_font_name(&self.font_name);
        self.stretch = FontStretch::from_font_name(&self.font_name);
        self.stem_v = self.font_weight.stem_v();

        if let Some(v) = font_post {
            self.italic_angle = v.italic_angle;
        }

        self.font_bbox.push(font_head.global_bbox.x_min as f32);
        self.font_bbox.push(font_head.global_bbox.y_min as f32);
        self.font_bbox.push(font_head.global_bbox.x_max as f32);
        self.font_bbox.push(font_head.global_bbox.y_max as f32);

        if let Some(v) = font_os2 {
            self.cap_height = v.capital_height().unwrap_or_default() as f32;
        }

        // Basic OTF detection, maybe there's a better solution
        if font_path.contains(".otf") {
            self.subsubtype = Some(FontSubsubtype::OpenType);
        }

        info!("Font info extracted with success");
    }
}

impl IdentifiedObject for FontDescriptor {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
            font_name: "Unknown".to_string(),
            font_family: "Unknown".to_string(),
            flags: 0,
            font_bbox: Vec::new(),
            italic_angle: 0.0,
            ascent: 0.0,
            descent: 0.0,
            cap_height: 0.0,
            stem_v: 0.0,
            font_weight: FontWeight::Normal,
            stretch: FontStretch::Normal,
            subtype: FontSubtype::TrueType,
            subsubtype: None,
            font_file: document.new_object_id(),
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

impl ObjectExport for FontDescriptor {
    fn to_object(&self) -> Object {
        let mut base_dic = dictionary! {
            "Type" => "FontDescriptor",
            "FontName" => Object::String(self.font_name.as_bytes().to_vec(), StringFormat::Literal),
            "FontFamily" => Object::String(self.font_family.as_bytes().to_vec(), StringFormat::Literal),
            "FontWeight" => self.font_weight.to_object(),
            "ItalicAngle" => self.italic_angle,
            "Ascent" => self.ascent,
            "Descent" => self.descent,
            "CapHeight" => self.cap_height,
            "StemV" => self.stem_v,
            "Flags" => 0
        };

        let mut font_bbox = Vec::new();
        for value in &self.font_bbox {
            font_bbox.push(Object::Real(*value));
        }

        base_dic.set("FontBBox", Object::Array(font_bbox));

        if self.subsubtype.is_some() {
            base_dic.set("FontFile3", Object::Reference(self.font_file));
        } else {
            base_dic.set("FontFile2", Object::Reference(self.font_file));
        }

        Dictionary(base_dic)
    }
}

impl PdfIntegrate for FontDescriptor {
    fn integrate_into_document(&self, document: &mut Document) {
        info!("Document <- FontDescriptor ({} {} R)", self.id.0, self.id.1);
        document.objects.insert(self.id, self.to_object());
    }
}

#[cfg(test)]
mod test {
    use crate::fude::Fude;
    use crate::pdf::font::{FontDescriptor, FontWeight};
    use crate::traits::IdentifiedObject;

    #[test]
    fn read_font() {
        let mut fude = Fude::new();
        let mut fd = FontDescriptor::from_document(&mut fude.inner_doc);

        fd.read_font("assets/fonts/OpenSans_Condensed-ExtraBold.ttf");

        assert_eq!(fd.font_name, "Open Sans Condensed ExtraBold");
        assert_eq!(fd.font_family, "Open Sans Condensed ExtraBold");
        assert!(matches!(fd.font_weight, FontWeight::ExtraBold));
        assert_eq!(fd.stem_v, 201.0);
        assert_eq!(fd.italic_angle, 0.0);
        assert_eq!(fd.cap_height, 1462.0);
        assert_eq!(fd.font_bbox, [-1157.0, -718.0, 2410.0, 2286.0]);
        assert_eq!(fd.stretch.to_string(), "Condensed");
    }
}

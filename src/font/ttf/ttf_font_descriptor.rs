use std::error::Error;

use lopdf::{dictionary, Dictionary, Object, ObjectId, Stream};
use rusttype::{Font, GlyphId, Scale};

use super::ttf_font_file::TtfFontFile;

pub struct TtfFontDescriptor {
    ascent: i64,
    descent: i64,
    avg_width: i64,
    cap_height: i64,
    font_bbox: Vec<Object>,
    font_name: String,
    italic_angle: i64,
    max_width: i64,
    stem_v: i64,
    font_file: Option<TtfFontFile>,
    id: ObjectId,
    file_id: ObjectId,
}

impl TtfFontDescriptor {
    pub fn new(ids: (ObjectId, ObjectId)) -> Self {
        Self {
            ascent: 0,
            descent: 0,
            avg_width: 0,
            cap_height: 0,
            font_bbox: Vec::new(),
            font_name: String::new(),
            italic_angle: 0,
            max_width: 0,
            stem_v: 0,
            font_file: None,
            id: ids.0,
            file_id: ids.1,
        }
    }

    pub fn finalize(&mut self, font_bin: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let scale = Scale::uniform(1000.0);
        let rtty = Font::try_from_bytes(&font_bin).unwrap();
        rtty.scale_for_pixel_height(1000.0);

        let glyph_count = rtty.glyph_count();
        let v_metrics = rtty.v_metrics(scale);

        let mut min_x: i64 = 0;
        let mut max_x: i64 = 0;

        self.ascent = v_metrics.ascent as i64;
        self.descent = v_metrics.descent as i64;
        self.cap_height = self.ascent;
        self.font_name = "OpenSans-Regular".to_string();

        for i in 0..glyph_count as u16 {
            let w = rtty.glyph(GlyphId(i)).scaled(scale);
            let h_metrics = w.h_metrics();

            let bearing = h_metrics.left_side_bearing as i64;
            let advance = h_metrics.advance_width as i64;

            if min_x > bearing {
                min_x = bearing;
            }

            if max_x < advance {
                max_x = advance;
            }

            self.avg_width = ((advance - self.avg_width) / (i as i64 + 1)) + self.avg_width;
        }

        self.max_width = max_x;
        self.font_bbox = vec![
            Object::Integer(min_x),
            Object::Integer(self.descent),
            Object::Integer(max_x),
            Object::Integer(self.ascent),
        ];
        self.italic_angle = 0;
        self.stem_v = 0;

        let file = TtfFontFile::from_bytes(font_bin, self.file_id);
        self.font_file = Some(file);

        Ok(())
    }

    pub fn to_object(&self) -> Object {
        Object::Reference(self.id)
    }

    pub fn to_dic(&self) -> Dictionary {
        dictionary! {
            "Ascent" => self.ascent,
            "AvgWidth" => self.avg_width,
            "CapHeight" => self.cap_height,
            "Descent" => self.descent,
            "Flags" => 3 as i64,
            "FontBBox" => self.font_bbox.clone(),
            "FontFile2" => self.font_file.as_ref().unwrap().to_object(),
            "FontName" => self.font_name.clone(),
            "ItalicAngle" => self.italic_angle,
            "MaxWidth" => self.max_width,
            "StemV" => self.stem_v,
            "Tpye" => "FontDescriptor"
        }
    }

    pub fn export(&self) -> (ObjectId, Dictionary) {
        (self.id, self.to_dic())
    }

    pub fn get_file(&self) -> (Object, Stream) {
        (
            self.font_file.as_ref().unwrap().to_object(),
            self.font_file.as_ref().unwrap().to_stream(),
        )
    }
}

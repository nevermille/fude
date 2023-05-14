use allsorts::binary::read::ReadScope;
use allsorts::font::read_cmap_subtable;
use allsorts::font_data::FontData;
use allsorts::tables::cmap::Cmap;
use allsorts::tables::FontTableProvider;
use allsorts::tag;
use lopdf::{dictionary, Dictionary, Object, ObjectId, Stream};
use rusttype::{Font, GlyphId, Scale};
use std::error::Error;
use std::io::ErrorKind;
use std::path::Path;

use crate::font::font_widths::FontWidths;

use super::TtfFontDescriptor;

pub struct TtfFont {
    used_text: String,
    font_file: String,
    embed: FontEmbedding,
    font_binary: Vec<u8>,
    id: ObjectId,
    widths: FontWidths,
    first_char: usize,
    last_char: usize,
    font_descriptor: TtfFontDescriptor,
}

pub enum FontEmbedding {
    Subset,
    Complete,
    External,
}

impl TtfFont {
    pub fn from_file(
        file: &str,
        ids: (ObjectId, ObjectId, ObjectId, ObjectId),
    ) -> Result<Self, std::io::Error> {
        if !Path::new(file).exists() {
            return Err(std::io::Error::new(
                ErrorKind::NotFound,
                "Font file not found",
            ));
        }

        Ok(Self {
            used_text: String::new(),
            font_file: file.to_string(),
            embed: FontEmbedding::Subset,
            font_binary: Vec::new(),
            id: ids.0,
            widths: FontWidths::new(ids.1),
            first_char: 0,
            last_char: 0,
            font_descriptor: TtfFontDescriptor::new((ids.2, ids.3)),
        })
    }

    pub fn add_text(&mut self, text: &str) {
        if let FontEmbedding::Subset = self.embed {
            self.used_text = format!("{}{}", self.used_text, text);
        }
    }

    /// Subsets the font
    ///
    /// I'd like to thank allsorts-tools for this part (https://github.com/yeslogic/allsorts-tools)
    fn subset(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Opening and reading font file
        let font_contents = std::fs::read(&self.font_file)?;
        let font_handler = ReadScope::new(&font_contents).read::<FontData>()?;
        let font_provider = font_handler.table_provider(0)?;

        // Glyph table extraction
        let cmap_data = font_provider.read_table_data(tag::CMAP)?;
        let cmap = ReadScope::new(&cmap_data).read::<Cmap>()?;
        let cmap_subtable = read_cmap_subtable(&cmap)?;

        if cmap_subtable.is_none() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "No suitable cmap sub-table found",
            )));
        }

        let (_, cmap_subtable) = cmap_subtable.unwrap();

        // Detecting all glyphs to export
        let mut glyphs_to_export: Vec<u16> = Vec::new();

        for ch in self.used_text.chars() {
            // Detect next glyph
            let extracted_glyph = cmap_subtable.map_glyph(ch as u32)?;

            if extracted_glyph.is_none() {
                continue;
            }

            let extracted_glyph_id = extracted_glyph.unwrap();
            glyphs_to_export.push(extracted_glyph_id);
        }

        // Remove duplicates
        glyphs_to_export.sort();
        glyphs_to_export.dedup();

        // Subset font
        Ok(allsorts::subset::subset(&font_provider, &glyphs_to_export)?)
    }

    pub fn finalize(&mut self) -> Result<(), Box<dyn Error>> {
        let font_bin = self.subset()?;
        let scale = Scale::uniform(1000.0);
        let rtty = Font::try_from_bytes(&font_bin).unwrap();
        rtty.scale_for_pixel_height(1000.0);

        let glyph_count = rtty.glyph_count();

        self.first_char = 1;
        self.last_char = glyph_count;

        for i in 0..glyph_count as u16 {
            let w = rtty.glyph(GlyphId(i)).scaled(scale);
            self.widths.add_width(w.h_metrics().advance_width as i64);
        }

        Ok(())
    }

    pub fn to_object(&self) -> Object {
        Object::Reference(self.id)
    }

    pub fn to_dic(&self) -> Dictionary {
        dictionary! {
            "BaseFont" => "OpenSans-Regular",
            "FirstChar" => Object::Integer(self.first_char as i64),
            "LastChar" => Object::Integer(self.last_char as i64),
            "FontDescriptor" => self.font_descriptor.to_object(),
            "Subtype" => "TrueType",
            "Type" => "Font",
            "Widths" => self.widths.to_object()
        }
    }

    pub fn get_descriptor(&self) -> (ObjectId, Object) {
        let descriptor = self.font_descriptor.export();

        (descriptor.0, Object::Dictionary(descriptor.1))
    }

    pub fn get_file(&self) -> (Object, Stream) {
        self.font_descriptor.get_file()
    }
}

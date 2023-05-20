mod font_descriptor;
mod font_stretch;
mod font_subsubtype;
/// Font subtypes
mod font_subtype;
mod font_weight;

use crate::macros::log::{error, info, warning};
use crate::traits::{IdentifiedObject, ObjectExport, PdfIntegrate};
use allsorts::binary::read::ReadScope;
use allsorts::font_data::FontData;
use allsorts::tables::cmap::Cmap;
use allsorts::tables::FontTableProvider;
use allsorts::tag;
pub use font_descriptor::FontDescriptor;
pub use font_stretch::FontStretch;
pub use font_subsubtype::FontSubsubtype;
pub use font_subtype::FontSubtype;
pub use font_weight::FontWeight;
use lopdf::Object::Dictionary;
use lopdf::{dictionary, Document, Object, ObjectId, Stream, StringFormat};

/// A font object
pub struct Font {
    /// The object's id
    pub id: ObjectId,

    /// The type of font
    pub subtype: FontSubtype,

    /// A font descriptor describing the font’s metrics other than its glyph widths
    pub font_descriptor: FontDescriptor,

    /// The font's path
    pub file: String,

    /// The text for this font
    pub associated_text: String,
}

impl Font {
    /// Subsets font with the associated text
    pub fn subset(&self) -> Vec<u8> {
        info!("Try to read font");

        // Opening and reading font file
        let raw_data = std::fs::read(&self.file);
        if let Err(e) = raw_data {
            error!("Unable to open font file: {}", e);
            return Vec::new();
        }
        let raw_data = raw_data.unwrap();

        info!("Font read, try to parse font");

        let font_handler = ReadScope::new(&raw_data).read::<FontData>();
        if let Err(e) = font_handler {
            error!("Unable to parse font file: {}", e);
            return raw_data.clone();
        }
        let font_handler = font_handler.unwrap();

        let font_provider = font_handler.table_provider(0);
        if let Err(e) = font_provider {
            error!("Unable to parse font file: {}", e);
            return raw_data.clone();
        }
        let font_provider = font_provider.unwrap();

        info!("Font parsed, try to read CMAP table");

        // Glyph table extraction
        let cmap_data = font_provider.read_table_data(tag::CMAP);
        if let Err(e) = cmap_data {
            error!("Unable to read CMAP: {}", e);
            return raw_data.clone();
        }
        let cmap_data = cmap_data.unwrap();

        let cmap = ReadScope::new(&cmap_data).read::<Cmap>();
        if let Err(e) = cmap {
            error!("Unable to read CMAP: {}", e);
            return raw_data.clone();
        }
        let cmap = cmap.unwrap();

        let cmap_subtable = allsorts::font::read_cmap_subtable(&cmap);
        if let Err(e) = cmap_subtable {
            error!("Unable to read CMAP: {}", e);
            return raw_data.clone();
        }

        let cmap_subtable = cmap_subtable.unwrap();
        if cmap_subtable.is_none() {
            error!("No CMAP table found");
            return raw_data.clone();
        }
        let (_, cmap_subtable) = cmap_subtable.unwrap();

        // Detecting all glyphs to export
        let mut glyphs_to_export: Vec<u16> = Vec::new();

        for ch in self.associated_text.chars() {
            // Detect next glyph
            let extracted_glyph = cmap_subtable.map_glyph(ch as u32);
            if let Err(e) = extracted_glyph {
                warning!("Error while extracting char glyph {}: {}", ch, e);
                continue;
            }
            let extracted_glyph = extracted_glyph.unwrap();

            if extracted_glyph.is_none() {
                warning!("No glyph found for char {}", ch);
                continue;
            }
            let extracted_glyph_id = extracted_glyph.unwrap();

            // Add glyph if it's not already present
            if !glyphs_to_export.contains(&extracted_glyph_id) {
                glyphs_to_export.push(extracted_glyph_id);
            }
        }

        info!("Found {} glyphs to export", glyphs_to_export.len());

        // Subset font
        let subsetted_font = allsorts::subset::subset(&font_provider, &glyphs_to_export);
        if let Err(e) = subsetted_font {
            error!("Error while subsetting font: {}", e);
            return raw_data.clone();
        }
        subsetted_font.unwrap()
    }
}

impl IdentifiedObject for Font {
    fn from_document(document: &mut Document) -> Self {
        Self {
            id: document.new_object_id(),
            subtype: FontSubtype::Type1,
            font_descriptor: FontDescriptor::from_document(document),
            file: String::new(),
            associated_text: String::new(),
        }
    }

    fn to_object_id(&self) -> ObjectId {
        self.id
    }
}

impl ObjectExport for Font {
    fn to_object(&self) -> Object {
        Dictionary(dictionary! {
            "Type" => "Font",
            "Subtype" => self.subtype.to_object(),
            "BaseFont" => Object::String(self.font_descriptor.font_name.as_bytes().to_vec(), StringFormat::Literal),
            "FirstChar" => 1, // TODO
            "LastChar" => 255, // TODO
            "FontDescriptor" => self.font_descriptor.to_object()
        })
    }
}

impl PdfIntegrate for Font {
    fn integrate_into_document(&self, document: &mut Document) {
        self.font_descriptor.integrate_into_document(document);
        document.objects.insert(self.id, self.to_object());

        // Embed font
        let subsetted_font = self.subset();
        let mut font_dic = dictionary! {"Length1" => subsetted_font.len() as i64};

        if let Some(v) = &self.font_descriptor.subsubtype {
            font_dic.set("Subtype", v.to_object());
        }

        let mut font_stream = Stream::new(font_dic, subsetted_font);
        font_stream.allows_compression = false;

        document
            .objects
            .insert(self.font_descriptor.font_file, Object::Stream(font_stream));
    }
}

use crate::traits::ObjectExport;
use lopdf::{Object, StringFormat};

pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl FontStretch {
    /// Determines font stretch from name
    ///
    /// Works only if font name has a common format
    ///
    /// # Parameters
    ///
    /// * `name`: The font's name
    pub fn from_font_name(name: &str) -> Self {
        let upper_name = name.to_uppercase();

        // Ultra Condensed
        if upper_name.contains("ULTRA CONDENSED") || upper_name.contains("ULTRACONDENSED") {
            return Self::UltraCondensed;
        }

        // Extra Condensed
        if upper_name.contains("EXTRA CONDENSED") || upper_name.contains("EXTRACONDENSED") {
            return Self::ExtraCondensed;
        }

        // Semi Condensed
        if upper_name.contains("SEMI CONDENSED")
            || upper_name.contains("SEMICONDENSED")
            || upper_name.contains("DEMI CONDENSED")
            || upper_name.contains("DEMICONDENSED")
        {
            return Self::SemiCondensed;
        }

        // Condensed
        if upper_name.contains("CONDENSED") {
            return Self::Condensed;
        }

        // Semi Expanded
        if upper_name.contains("SEMI EXPANDED")
            || upper_name.contains("SEMIEXPANDED")
            || upper_name.contains("DEMI EXPANDED")
            || upper_name.contains("DEMIEXPANDED")
        {
            return Self::SemiExpanded;
        }

        // Extra Expanded
        if upper_name.contains("EXTRA EXPANDED") || upper_name.contains("EXTRAEXPANDED") {
            return Self::ExtraExpanded;
        }

        // Ultra Expanded
        if upper_name.contains("ULTRA EXPANDED") || upper_name.contains("ULTRAEXPANDED") {
            return Self::UltraExpanded;
        }

        // Expanded
        if upper_name.contains("EXPANDED") {
            return Self::Expanded;
        }

        Self::Normal
    }
}

impl ToString for FontStretch {
    fn to_string(&self) -> String {
        match self {
            Self::UltraCondensed => "UltraCondensed".to_string(),
            Self::ExtraCondensed => "ExtraCondensed".to_string(),
            Self::Condensed => "Condensed".to_string(),
            Self::SemiCondensed => "SemiCondensed".to_string(),
            Self::Normal => "Normal".to_string(),
            Self::SemiExpanded => "SemiExpanded".to_string(),
            Self::Expanded => "Expanded".to_string(),
            Self::ExtraExpanded => "ExtraExpanded".to_string(),
            Self::UltraExpanded => "UltraExpanded".to_string(),
        }
    }
}

impl ObjectExport for FontStretch {
    fn to_object(&self) -> Object {
        Object::String(self.to_string().as_bytes().to_vec(), StringFormat::Literal)
    }
}

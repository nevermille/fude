use crate::traits::ObjectExport;
use lopdf::Object;

pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack,
}

impl FontWeight {
    /// Converts the font weight into numerical value
    pub fn numerical_value(&self) -> f32 {
        match self {
            Self::Thin => 100.0,
            Self::ExtraLight => 200.0,
            Self::Light => 300.0,
            Self::Normal => 400.0,
            Self::Medium => 500.0,
            Self::SemiBold => 600.0,
            Self::Bold => 700.0,
            Self::ExtraBold => 800.0,
            Self::Black => 900.0,
            Self::ExtraBlack => 950.0,
        }
    }

    /// Gives a stem_v value for the font's weight
    pub fn stem_v(&self) -> f32 {
        match self {
            Self::Thin | Self::ExtraLight => 50.0,
            Self::Light => 71.0,
            Self::Normal => 109.0,
            Self::Medium => 125.0,
            Self::SemiBold => 135.0,
            Self::Bold => 165.0,
            Self::ExtraBold => 201.0,
            Self::Black | Self::ExtraBlack => 241.0,
        }
    }

    /// Determines font weight from name
    ///
    /// Works only if font name has a common format
    ///
    /// # Parameters
    ///
    /// * `name`: The font's name
    pub fn from_font_name(name: &str) -> Self {
        let upper_name = name.to_uppercase();

        // Thin
        if upper_name.contains("THIN") || upper_name.contains("HAIRLINE") {
            return Self::Thin;
        }

        // Extra Light
        if upper_name.contains("EXTRA LIGHT")
            || upper_name.contains("EXTRALIGHT")
            || upper_name.contains("ULTRA LIGHT")
            || upper_name.contains("ULTRALIGHT")
        {
            return Self::ExtraLight;
        }

        // Light (Must be after Extra Light check)
        if upper_name.contains("LIGHT") {
            return Self::Thin;
        }

        // Normal
        if upper_name.contains("NORMAL")
            || upper_name.contains("REGULAR")
            || upper_name.contains("BOOK")
        {
            return Self::Normal;
        }

        // Medium
        if upper_name.contains("MEDIUM") {
            return Self::Medium;
        }

        // Semi Bold
        if upper_name.contains("SEMI BOLD")
            || upper_name.contains("SEMIBOLD")
            || upper_name.contains("DEMI BOLD")
            || upper_name.contains("DEMIBOLD")
        {
            return Self::SemiBold;
        }

        // Extra Bold
        if upper_name.contains("EXTRA BOLD")
            || upper_name.contains("EXTRABOLD")
            || upper_name.contains("ULTRA BOLD")
            || upper_name.contains("ULTRABOLD")
        {
            return Self::ExtraBold;
        }

        // Bold (Must be after other X Bold checks)
        if upper_name.contains("BOLD") {
            return Self::Bold;
        }

        // Extra Black
        if upper_name.contains("EXTRA BLACK")
            || upper_name.contains("EXTRABLACK")
            || upper_name.contains("ULTRA BLACK")
            || upper_name.contains("ULTRABLACK")
        {
            return Self::ExtraBlack;
        }

        // Black (Must be after Black check)
        if upper_name.contains("BLACK") || upper_name.contains("HEAVY") {
            return Self::Black;
        }

        // If nothing found
        Self::Normal
    }
}

impl ObjectExport for FontWeight {
    fn to_object(&self) -> Object {
        Object::Real(self.numerical_value())
    }
}

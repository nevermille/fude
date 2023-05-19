pub struct FontBuilder {
    pub path: String,
}

impl FontBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            path: String::new(),
        }
    }
}

impl Default for FontBuilder {
    fn default() -> Self {
        Self::new()
    }
}

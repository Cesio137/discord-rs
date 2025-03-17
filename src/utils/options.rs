pub struct Options {
    pub intents: u32,
    pub compress: bool,
    pub large_threshold: u8,
}

impl Options {
    fn default() -> Self {
        Self {
            intents: 513,
            compress: false,
            large_threshold: 250,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::default()
    }
}

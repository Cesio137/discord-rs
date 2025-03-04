pub struct Options {
    pub intents: u32,
    pub partials: u32,
}

impl Options {
    fn default() -> Self {
        Self {
            intents: 513,
            partials: 2,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::default()
    }
}
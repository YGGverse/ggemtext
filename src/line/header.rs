pub mod gemtext;
pub mod level;

pub use gemtext::Gemtext;
pub use level::Level;

/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) entity holder
pub struct Header {
    pub level: Level,
    pub value: String,
}

impl Header {
    // Constructors

    /// Parse `Self` from line string
    pub fn parse(line: &str) -> Option<Self> {
        if let Some(value) = line.as_h1_value() {
            return Some(Self {
                level: Level::H1,
                value: value.to_string(),
            });
        }
        if let Some(value) = line.as_h2_value() {
            return Some(Self {
                level: Level::H2,
                value: value.to_string(),
            });
        }
        if let Some(value) = line.as_h3_value() {
            return Some(Self {
                level: Level::H3,
                value: value.to_string(),
            });
        }
        None
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn to_source(&self) -> String {
        self.value.to_source(&self.level)
    }
}

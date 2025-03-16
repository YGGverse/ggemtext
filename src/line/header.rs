/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) tag
/// * store as entire static chars array
pub const TAG_H1: &str = "#";
pub const TAG_H2: &str = "##";
pub const TAG_H3: &str = "###";

/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) type holder
pub enum Level {
    H1,
    H2,
    H3,
}

/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) entity holder
pub struct Header {
    pub level: Level,
    pub value: String,
}

impl Header {
    // Constructors

    /// Parse `Self` from line string
    pub fn parse(line: &str) -> Option<Self> {
        Some(Self {
            level: line.to_level()?,
            value: line.as_value()?.to_string(),
        })
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn to_source(&self) -> String {
        self.value.to_source(&self.level)
    }
}

pub trait Gemtext {
    /// Get [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) value for `Self`
    fn as_value(&self) -> Option<&Self>;
    /// Convert `Self` to `Level`
    fn to_level(&self) -> Option<Level>;
    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    fn to_source(&self, level: &Level) -> String;
}

impl Gemtext for str {
    fn as_value(&self) -> Option<&str> {
        if let Some(h3) = self.strip_prefix(TAG_H3) {
            if h3.trim_start().starts_with(TAG_H1) {
                return None; // H4+
            }
            return Some(h3.trim());
        }
        if let Some(h2) = self.strip_prefix(TAG_H2) {
            return Some(h2.trim());
        }
        if let Some(h1) = self.strip_prefix(TAG_H1) {
            return Some(h1.trim());
        }
        None
    }
    fn to_level(&self) -> Option<Level> {
        if let Some(h3) = self.strip_prefix(TAG_H3) {
            if h3.trim_start().starts_with(TAG_H1) {
                return None; // H4+
            }
            return Some(Level::H3);
        }
        if self.starts_with(TAG_H2) {
            return Some(Level::H2);
        }
        if self.starts_with(TAG_H1) {
            return Some(Level::H1);
        }
        None
    }
    fn to_source(&self, level: &Level) -> String {
        format!(
            "{} {}",
            match level {
                Level::H1 => TAG_H1,
                Level::H2 => TAG_H2,
                Level::H3 => TAG_H3,
            },
            self.trim()
        )
    }
}

use super::Level;

pub trait Gemtext {
    /// Get [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) value for `Self`
    fn as_value(&self) -> Option<&str>;
    /// Get parsed H1 header value for `Self`
    fn as_h1_value(&self) -> Option<&str>;
    /// Get parsed H2 header value `Self`
    fn as_h2_value(&self) -> Option<&str>;
    /// Get parsed H3 header value `Self`
    fn as_h3_value(&self) -> Option<&str>;
    /// Get parsed header value `Self` match `Level`
    fn as_value_match_level(&self, level: Level) -> Option<&str>;
    /// Convert `Self` to `Level`
    fn to_level(&self) -> Option<Level>;
    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    fn to_source(&self, level: &Level) -> String;
}

impl Gemtext for str {
    fn as_value(&self) -> Option<&str> {
        if let Some(value) = self.as_h1_value() {
            return Some(value);
        }
        if let Some(value) = self.as_h2_value() {
            return Some(value);
        }
        if let Some(value) = self.as_h3_value() {
            return Some(value);
        }
        None
    }
    fn as_h1_value(&self) -> Option<&str> {
        self.as_value_match_level(Level::H1)
    }
    fn as_h2_value(&self) -> Option<&str> {
        self.as_value_match_level(Level::H2)
    }
    fn as_h3_value(&self) -> Option<&str> {
        self.as_value_match_level(Level::H3)
    }
    fn as_value_match_level(&self, level: Level) -> Option<&str> {
        if let Some(postfix) = self.strip_prefix(level.as_tag()) {
            let value = postfix.trim();
            if value.starts_with(Level::H1.as_tag()) {
                return None;
            }
            return Some(value);
        }
        None
    }
    fn to_level(&self) -> Option<Level> {
        if self.as_h1_value().is_some() {
            return Some(Level::H1);
        }
        if self.as_h2_value().is_some() {
            return Some(Level::H2);
        }
        if self.as_h3_value().is_some() {
            return Some(Level::H3);
        }
        None
    }
    fn to_source(&self, level: &Level) -> String {
        format!("{} {}", level.as_tag(), self.trim())
    }
}

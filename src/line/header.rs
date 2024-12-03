use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) type holder
pub enum Level {
    H1,
    H2,
    H3,
}

/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) entity holder
pub struct Header {
    pub value: String,
    pub level: Level,
}

impl Header {
    // Constructors

    /// Parse `Self` from line string
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^(#{1,3})\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect header level
        let level = regex.get(1)?;

        let level = match level.len() {
            1 => Level::H1,
            2 => Level::H2,
            3 => Level::H3,
            _ => return None,
        };

        // Detect header value
        let value = regex.get(2)?.trim();

        if value.is_empty() {
            return None;
        }

        // Result
        Some(Self {
            level,
            value: value.to_string(),
        })
    }
}

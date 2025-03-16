use super::TAG;
use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// Inline [preformatted](https://geminiprotocol.net/docs/gemtext-specification.gmi#in-pre-formatted-mode) entity holder
pub struct Inline {
    pub value: String,
}

impl Inline {
    // Constructors

    /// Parse `Self` from line string
    pub fn from(line: &str) -> Option<Self> {
        // Skip next operations on prefix and postfix mismatch `TAG`
        // * replace regex implementation @TODO
        if !line.starts_with(TAG) && !line.ends_with(TAG) {
            return None;
        }

        // Parse line
        let regex = Regex::split_simple(
            r"^`{3}([^`]+)`{3}$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Extract formatted value
        Some(Self {
            value: regex.get(1)?.trim().to_string(),
        })
    }
}

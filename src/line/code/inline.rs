use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// Inline [preformatted](https://geminiprotocol.net/docs/gemtext-specification.gmi#in-pre-formatted-mode) entity holder
pub struct Inline {
    pub value: String,
}

impl Inline {
    // Constructors

    /// Parse `Self` from line string
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^`{3}([^`]*)`{3}$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect value
        let value = regex.get(1)?;

        if value.trim().is_empty() {
            return None;
        }

        // Result
        Some(Self {
            value: value.to_string(),
        })
    }
}

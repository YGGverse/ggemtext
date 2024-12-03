use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// [Quote](https://geminiprotocol.net/docs/gemtext-specification.gmi#quote-lines) entity holder
pub struct Quote {
    pub value: String,
}

impl Quote {
    // Constructors

    /// Parse `Self` from line string
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^>\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Extract formatted value
        let value = regex.get(1)?.trim().to_string();

        // Result
        Some(Self { value })
    }
}

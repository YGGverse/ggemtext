use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// [List](https://geminiprotocol.net/docs/gemtext-specification.gmi#list-items) entity holder
pub struct List {
    pub value: String,
}

impl List {
    // Constructors

    /// Parse `Self` from string
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^\*\s*(.+)$",
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

use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// [List item](https://geminiprotocol.net/docs/gemtext-specification.gmi#list-items)
pub struct List {
    pub value: String,
}

impl List {
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

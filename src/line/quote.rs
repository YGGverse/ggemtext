use glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags};

pub struct Quote {
    pub value: GString,
}

impl Quote {
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^>\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect value
        let value = regex.get(1)?;

        // Result
        Some(Self {
            value: GString::from(value.as_str()),
        })
    }
}

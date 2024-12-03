use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

pub struct Quote {
    pub value: String,
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
        let value = regex.get(1)?.trim();

        // Result
        Some(Self {
            value: String::from(value),
        })
    }
}

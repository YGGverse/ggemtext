use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

pub struct Inline {
    pub value: String,
}

impl Inline {
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

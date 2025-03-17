pub mod gemtext;
pub use gemtext::Gemtext;

use super::TAG;

/// Inline [preformatted](https://geminiprotocol.net/docs/gemtext-specification.gmi#in-pre-formatted-mode) entity holder
pub struct Inline {
    pub value: String,
}

impl Inline {
    // Constructors

    /// Parse `Self` from line string
    pub fn parse(line: &str) -> Option<Self> {
        line.as_value().map(|v| Self {
            value: v.to_string(),
        })
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn to_source(&self) -> String {
        self.value.to_source()
    }
}

#[test]
fn test() {
    fn assert(source: &str, value: &str) {
        let list = Inline::parse(source).unwrap();
        assert_eq!(list.value, value);
        assert_eq!(list.to_source(), format!("```{value}```"));
    }
    assert("```inline```", "inline");
    assert("```inline ```", "inline");
    assert("``` inline ```", "inline");
    assert("``` inline```", "inline");
    assert("``` inline``` ", "inline");
    assert("``````inline``` ", "```inline");
    assert("``````inline`````` ", "```inline```");
    assert("```inline`````` ", "inline```");
    assert!("```inline".as_value().is_none());
    assert!("```inline``` ne".as_value().is_none());
}

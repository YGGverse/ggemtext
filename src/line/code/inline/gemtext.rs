use super::TAG;

pub trait Gemtext {
    /// Get [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) value for `Self`
    fn as_value(&self) -> Option<&str>;
    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    fn to_source(&self) -> String;
}

impl Gemtext for str {
    fn as_value(&self) -> Option<&str> {
        if let Some(p) = self.strip_prefix(TAG) {
            return p.trim().strip_suffix(TAG).map(|s| s.trim());
        }
        None
    }
    fn to_source(&self) -> String {
        format!("{TAG}{}{TAG}", self.trim())
    }
}

#[test]
fn test() {
    fn assert(source: &str, value: &str) {
        assert_eq!(source.as_value(), Some(value));
        assert_eq!(value.to_source(), format!("```{value}```"));
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

use super::TAG;

pub trait Gemtext {
    /// Get [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) value for `Self`
    fn as_value(&self) -> Option<&str>;
    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    fn to_source(&self) -> String;
}

impl Gemtext for str {
    fn as_value(&self) -> Option<&str> {
        self.strip_prefix(TAG).map(|s| s.trim())
    }
    fn to_source(&self) -> String {
        format!("{TAG} {}", self.trim())
    }
}

#[test]
fn test() {
    const SOURCE: &str = "* Item";
    const VALUE: &str = "Item";

    assert_eq!(SOURCE.as_value(), Some(VALUE));
    assert_eq!(VALUE.to_source(), SOURCE)
}

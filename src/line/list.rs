/// [List item](https://geminiprotocol.net/docs/gemtext-specification.gmi#list-items) tag
pub const TAG: char = '*';

/// [List](https://geminiprotocol.net/docs/gemtext-specification.gmi#list-items) entity holder
pub struct List {
    pub value: String,
}

impl List {
    // Constructors

    /// Parse `Self` from [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn parse(line: &str) -> Option<Self> {
        Some(Self {
            value: line.as_value()?.to_string(),
        })
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn as_source(&self) -> String {
        self.value.to_source()
    }
}

pub trait Gemtext {
    /// Get [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) value from `Self`
    fn as_value(&self) -> Option<&Self>;
    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    fn to_source(&self) -> String;
}

impl Gemtext for str {
    fn as_value(&self) -> Option<&Self> {
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

    // test struct
    assert_eq!(List::parse(SOURCE).unwrap().value, VALUE);

    // test trait
    assert_eq!(SOURCE.as_value(), Some(VALUE));
    assert_eq!(VALUE.to_source(), SOURCE)
}

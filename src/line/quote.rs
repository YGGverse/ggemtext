/// [Quote item](https://geminiprotocol.net/docs/gemtext-specification.gmi#quote-lines) tag
pub const TAG: char = '>';

/// [Quote](https://geminiprotocol.net/docs/gemtext-specification.gmi#quote-lines) entity holder
pub struct Quote {
    pub value: String,
}

impl Quote {
    // Constructors

    /// Parse `Self` from [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn parse(line: &str) -> Option<Self> {
        Some(Self {
            value: line.as_value()?.to_string(),
        })
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn to_source(&self) -> String {
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
    const SOURCE: &str = "> Quote";
    const VALUE: &str = "Quote";

    // test `Quote`
    let quote = Quote::parse(SOURCE).unwrap();
    assert_eq!(quote.value, VALUE);
    assert_eq!(quote.to_source(), SOURCE);

    // test `Gemtext`
    assert_eq!(SOURCE.as_value(), Some(VALUE));
    assert_eq!(VALUE.to_source(), SOURCE)
}

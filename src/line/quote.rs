pub mod gemtext;
pub use gemtext::Gemtext;

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

#[test]
fn test() {
    const SOURCE: &str = "> Quote";
    const VALUE: &str = "Quote";

    let quote = Quote::parse(SOURCE).unwrap();

    assert_eq!(quote.value, VALUE);
    assert_eq!(quote.to_source(), SOURCE);
}

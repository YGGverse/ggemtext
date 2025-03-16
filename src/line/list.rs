pub mod gemtext;
pub use gemtext::Gemtext;

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
    pub fn to_source(&self) -> String {
        self.value.to_source()
    }
}

#[test]
fn test() {
    const SOURCE: &str = "* Item";
    const VALUE: &str = "Item";

    let list = List::parse(SOURCE).unwrap();
    assert_eq!(list.value, VALUE);
    assert_eq!(list.to_source(), SOURCE);
}

pub mod gemtext;
pub mod level;

pub use gemtext::Gemtext;
pub use level::Level;

/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) entity holder
pub struct Header {
    pub level: Level,
    pub value: String,
}

impl Header {
    // Constructors

    /// Parse `Self` from line string
    pub fn parse(line: &str) -> Option<Self> {
        if let Some(value) = line.as_h1_value() {
            return Some(Self {
                level: Level::H1,
                value: value.to_string(),
            });
        }
        if let Some(value) = line.as_h2_value() {
            return Some(Self {
                level: Level::H2,
                value: value.to_string(),
            });
        }
        if let Some(value) = line.as_h3_value() {
            return Some(Self {
                level: Level::H3,
                value: value.to_string(),
            });
        }
        None
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn to_source(&self) -> String {
        self.value.to_source(&self.level)
    }
}

#[test]
fn test() {
    fn test(source: &str, value: &str, level: Level) {
        fn f(s: &str) -> String {
            s.chars().filter(|&c| c != ' ').collect()
        }
        let header = Header::parse(source).unwrap();
        assert_eq!(header.value, value);
        assert_eq!(header.level.as_tag(), level.as_tag());
        assert_eq!(f(&header.to_source()), f(source));
    }
    // h1
    test("# H1", "H1", Level::H1);
    test("# H1 ", "H1", Level::H1);
    test("#H1", "H1", Level::H1);
    test("#H1 ", "H1", Level::H1);
    // h2
    test("## H2", "H2", Level::H2);
    test("## H2 ", "H2", Level::H2);
    test("##H2", "H2", Level::H2);
    test("##H2 ", "H2", Level::H2);
    // h3
    test("### H3", "H3", Level::H3);
    test("### H3 ", "H3", Level::H3);
    test("###H3", "H3", Level::H3);
    test("###H3 ", "H3", Level::H3);
    // other
    assert!(Header::parse("H").is_none());
    assert!(Header::parse("#### H").is_none())
}

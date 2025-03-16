use super::Level;

pub trait Gemtext {
    /// Get [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) value for `Self`
    fn as_value(&self) -> Option<&str>;
    /// Get parsed H1 header value for `Self`
    fn as_h1_value(&self) -> Option<&str>;
    /// Get parsed H2 header value `Self`
    fn as_h2_value(&self) -> Option<&str>;
    /// Get parsed H3 header value `Self`
    fn as_h3_value(&self) -> Option<&str>;
    /// Get parsed header value `Self` match `Level`
    fn as_value_match_level(&self, level: Level) -> Option<&str>;
    /// Convert `Self` to `Level`
    fn to_level(&self) -> Option<Level>;
    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    fn to_source(&self, level: &Level) -> String;
}

impl Gemtext for str {
    fn as_value(&self) -> Option<&str> {
        if let Some(value) = self.as_h1_value() {
            return Some(value);
        }
        if let Some(value) = self.as_h2_value() {
            return Some(value);
        }
        if let Some(value) = self.as_h3_value() {
            return Some(value);
        }
        None
    }
    fn as_h1_value(&self) -> Option<&str> {
        self.as_value_match_level(Level::H1)
    }
    fn as_h2_value(&self) -> Option<&str> {
        self.as_value_match_level(Level::H2)
    }
    fn as_h3_value(&self) -> Option<&str> {
        self.as_value_match_level(Level::H3)
    }
    fn as_value_match_level(&self, level: Level) -> Option<&str> {
        self.strip_prefix(level.as_tag())
            .map(|postfix| postfix.trim())
            .filter(|value| !value.starts_with(Level::H1.as_tag()))
    }
    fn to_level(&self) -> Option<Level> {
        if self.as_h1_value().is_some() {
            return Some(Level::H1);
        }
        if self.as_h2_value().is_some() {
            return Some(Level::H2);
        }
        if self.as_h3_value().is_some() {
            return Some(Level::H3);
        }
        None
    }
    fn to_source(&self, level: &Level) -> String {
        format!("{} {}", level.as_tag(), self.trim())
    }
}

#[test]
fn test() {
    const VALUE: &str = "H";
    let mut value: Option<&str> = Some(VALUE);
    for t in ["#", "##", "###", "####"] {
        if t.len() > 3 {
            value = None;
        }
        assert_eq!(format!("{t}{VALUE}").as_value(), value);
        assert_eq!(format!("{t}{VALUE} ").as_value(), value);
        assert_eq!(format!("{t} {VALUE}").as_value(), value);
        assert_eq!(format!("{t} {VALUE} ").as_value(), value);
    }

    fn to_source(l: &Level) {
        assert_eq!(VALUE.to_source(l), format!("{} {VALUE}", l.as_tag()));
    }
    to_source(&Level::H1);
    to_source(&Level::H2);
    to_source(&Level::H3);

    fn to_level(l: &Level) {
        fn assert(s: String, l: &str) {
            assert_eq!(s.to_level().unwrap().as_tag(), l);
        }
        let t = l.as_tag();
        assert(format!("{t} {VALUE}"), t);
        assert(format!("{t} {VALUE} "), t);
        assert(format!("{t}{VALUE} "), t);
        assert(format!("{t} {VALUE} "), t);
    }
    to_level(&Level::H1);
    to_level(&Level::H2);
    to_level(&Level::H3);
}

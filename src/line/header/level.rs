/// [Header](https://geminiprotocol.net/docs/gemtext-specification.gmi#heading-lines) type holder
pub enum Level {
    H1,
    H2,
    H3,
}

impl Level {
    pub fn as_tag(&self) -> &str {
        match self {
            Level::H1 => "#",
            Level::H2 => "##",
            Level::H3 => "###",
        }
    }
}

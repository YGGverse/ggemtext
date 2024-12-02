use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Multiline(crate::line::code::multiline::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Multiline(e) => {
                write!(f, "Multiline error: {e}")
            }
        }
    }
}

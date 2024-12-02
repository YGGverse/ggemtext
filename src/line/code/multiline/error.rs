use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Completed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Completed => {
                write!(f, "Could not continue as completed!")
            }
        }
    }
}

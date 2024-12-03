pub mod error;
pub mod inline;
pub mod multiline;

pub use error::Error;
use inline::Inline;
use multiline::Multiline;

pub struct Code {
    // nothing yet..
}

impl Code {
    // Constructors

    /// Parse inline `Self` from string
    pub fn inline_from(line: &str) -> Option<Inline> {
        Inline::from(line)
    }

    /// Begin multi-line parse `Self` from string
    pub fn multiline_begin_from(line: &str) -> Option<Multiline> {
        Multiline::begin_from(line)
    }

    /// Continue multi-line parse `Self` from string
    pub fn multiline_continue_from(this: &mut Multiline, line: &str) -> Result<(), Error> {
        match Multiline::continue_from(this, line) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::Multiline(e)),
        }
    }
}

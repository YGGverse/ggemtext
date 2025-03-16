use super::TAG;

pub mod error;
pub use error::Error;

// Shared defaults

pub const NEW_LINE: char = '\n';

/// Multi-line [preformatted](https://geminiprotocol.net/docs/gemtext-specification.gmi#in-pre-formatted-mode) entity holder
pub struct Multiline {
    pub alt: Option<String>,
    pub value: String,
    pub completed: bool,
}

impl Multiline {
    // Constructors

    /// Search in line string for tag open,
    /// return Self constructed on success or None
    pub fn begin_from(line: &str) -> Option<Self> {
        if line.starts_with(TAG) {
            let alt = line.trim_start_matches(TAG).trim();

            return Some(Self {
                alt: match alt.is_empty() {
                    true => None,
                    false => Some(alt.to_string()),
                },
                value: String::new(),
                completed: false,
            });
        }

        None
    }

    /// Continue preformatted buffer from line string,
    /// set `completed` as True on close tag found
    pub fn continue_from(&mut self, line: &str) -> Result<(), Error> {
        // Make sure buffer not completed yet
        if self.completed {
            return Err(Error::Completed);
        }

        // Append to value, trim close tag on exists
        self.value.push_str(line.trim_end_matches(TAG));

        // Line contain close tag
        if line.ends_with(TAG) {
            self.completed = true;
        } else {
            self.value.push(NEW_LINE);
        }

        Ok(())
    }
}

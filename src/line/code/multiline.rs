pub mod error;
pub use error::Error;

pub const NEW_LINE: char = '\n';
pub const TAG: &str = "```";

pub struct Multiline {
    pub alt: Option<String>,
    pub value: String,
    pub completed: bool,
}

impl Multiline {
    /// Search in line for tag open,
    /// return Self constructed on success or None
    pub fn begin_from(line: &str) -> Option<Self> {
        if line.starts_with(TAG) {
            let alt = line.trim_start_matches(TAG).trim();

            return Some(Self {
                alt: match alt.is_empty() {
                    true => None,
                    false => Some(String::from(alt)),
                },
                value: String::new(),
                completed: false,
            });
        }

        None
    }

    /// Continue preformatted buffer from line,
    /// set `completed` as True on close tag found
    pub fn continue_from(&mut self, line: &str) -> Result<(), Error> {
        // Make sure buffer not completed yet
        if self.completed {
            return Err(Error::Completed);
        }

        // Line contain close tag
        if line.ends_with(TAG) {
            self.completed = true;
        }

        // Prepend new line before next lines only
        if !self.value.is_empty() {
            self.value.push(NEW_LINE);
        }

        // Append to value, trim close tag on exists
        self.value.push_str(line.trim_end_matches(TAG));

        Ok(())
    }
}

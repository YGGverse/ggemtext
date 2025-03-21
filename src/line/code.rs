pub mod error;
pub use error::Error;

pub const TAG: &str = "```";
pub const NEW_LINE: char = '\n';

/// Multi-line [preformatted](https://geminiprotocol.net/docs/gemtext-specification.gmi#in-pre-formatted-mode) entity holder
pub struct Code {
    pub alt: Option<String>,
    pub value: String,
    pub is_completed: bool,
}

impl Code {
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
                is_completed: false,
            });
        }
        None
    }

    /// Continue preformatted buffer from line string,
    /// set `completed` as True on close tag found
    pub fn continue_from(&mut self, line: &str) -> Result<(), Error> {
        // Make sure buffer not completed yet
        if self.is_completed {
            return Err(Error::Completed);
        }

        // Append to value, trim close tag on exists
        self.value.push_str(line.trim_end_matches(TAG));

        // Line contain close tag
        if line.ends_with(TAG) {
            self.is_completed = true;
        } else {
            self.value.push(NEW_LINE);
        }

        Ok(())
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) format
    pub fn to_source(&self) -> String {
        format!(
            "{TAG}{}{NEW_LINE}{}{TAG}",
            match &self.alt {
                Some(alt) => format!(" {}", alt.trim()),
                None => String::new(),
            },
            self.value
        )
    }
}

#[test]
fn test() {
    match Code::begin_from("```alt") {
        Some(mut code) => {
            assert!(code.continue_from("line 1").is_ok());
            assert!(code.continue_from("line 2").is_ok());
            assert!(code.continue_from("```").is_ok()); // complete

            assert!(code.is_completed);
            assert_eq!(code.alt, Some("alt".into()));
            assert_eq!(code.value.len(), 12 + 2); // +NL

            assert_eq!(
                code.to_source(),
                format!("{TAG} alt{NEW_LINE}line 1{NEW_LINE}line 2{NEW_LINE}{TAG}")
            )
        }
        None => assert!(false),
    }
}

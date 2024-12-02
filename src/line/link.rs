use glib::{DateTime, Regex, RegexCompileFlags, RegexMatchFlags, TimeZone, Uri, UriFlags};

pub struct Link {
    pub alt: Option<String>,         // [optional] alternative link description
    pub is_external: Option<bool>,   // [optional] external link indication, on base option provided
    pub timestamp: Option<DateTime>, // [optional] valid link DateTime object
    pub uri: Uri,                    // [required] valid link URI object
}

impl Link {
    pub fn from(line: &str, base: Option<&Uri>, timezone: Option<&TimeZone>) -> Option<Self> {
        // Define initial values
        let mut alt = None;
        let mut timestamp = None;
        let mut is_external = None;

        // Begin line parse
        let regex = Regex::split_simple(
            r"^=>\s*([^\s]+)\s*(\d{4}-\d{2}-\d{2})?\s*(.+)?$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect address required to continue
        let mut unresolved_address = regex.get(1)?.to_string();

        // Seems that [Uri resolver](https://docs.gtk.org/glib/type_func.Uri.resolve_relative.html)
        // does not support [protocol-relative URI](https://datatracker.ietf.org/doc/html/rfc3986#section-4.2)
        // resolve manually
        if unresolved_address.starts_with("//:") {
            let scheme = match base {
                Some(base) => base.scheme(),
                None => return None,
            };
            unresolved_address = unresolved_address.replace("//:", &format!("{scheme}://"));
        }

        // Convert address to the valid URI
        let uri = match base {
            // Base conversion requested
            Some(base_uri) => {
                // Convert relative address to absolute
                match Uri::resolve_relative(
                    Some(&base_uri.to_str()),
                    unresolved_address.as_str(),
                    UriFlags::NONE,
                ) {
                    Ok(resolved_str) => {
                        // Try convert string to the valid URI
                        match Uri::parse(&resolved_str, UriFlags::NONE) {
                            Ok(resolved_uri) => {
                                // Change external status
                                is_external = Some(resolved_uri.scheme() != base_uri.scheme());

                                // Result
                                resolved_uri
                            }
                            Err(_) => return None,
                        }
                    }
                    Err(_) => return None,
                }
            }
            // Base resolve not requested
            None => {
                // Just try convert address to valid URI
                match Uri::parse(&unresolved_address, UriFlags::NONE) {
                    Ok(unresolved_uri) => unresolved_uri,
                    Err(_) => return None,
                }
            }
        };

        // Timestamp
        if let Some(date) = regex.get(2) {
            // @TODO even possible, but simpler to work with `DateTime` API
            // await for new features in `Date` as better in Gemini context
            // https://docs.gtk.org/glib/struct.Date.html
            timestamp = match DateTime::from_iso8601(&format!("{date}T00:00:00"), timezone) {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        }

        // Alt
        if let Some(value) = regex.get(3) {
            if !value.is_empty() {
                alt = Some(value.to_string())
            }
        };

        Some(Self {
            alt,
            is_external,
            timestamp,
            uri,
        })
    }
}

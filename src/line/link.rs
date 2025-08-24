use glib::{DateTime, TimeZone, Uri, UriFlags};
const SEP: [char; 2] = [' ', '\t'];
const S: char = ' ';

pub const TAG: &str = "=>";

/// [Link](https://geminiprotocol.net/docs/gemtext-specification.gmi#link-lines) entity holder
pub struct Link {
    /// For performance reasons, hold Gemtext date and alternative together as the optional String
    /// * to extract valid [DateTime](https://docs.gtk.org/glib/struct.DateTime.html) use `time` implementation method
    pub alt: Option<String>,
    /// For performance reasons, hold URL as the raw String
    /// * to extract valid [Uri](https://docs.gtk.org/glib/struct.Uri.html) use `uri` implementation method
    pub url: String,
}

impl Link {
    // Constructors

    /// Parse `Self` from line string
    pub fn parse(line: &str) -> Option<Self> {
        let l = line.strip_prefix(TAG)?.trim_matches(&SEP);
        let u = l.find(|c: char| SEP.contains(&c)).map_or(l, |i| &l[..i]);
        if u.is_empty() {
            return None;
        }
        Some(Self {
            alt: l
                .get(u.len()..)
                .map(|a| a.trim())
                .filter(|a| !a.is_empty())
                .map(|a| a.to_string()),
            url: u.to_string(),
        })
    }

    // Converters

    /// Convert `Self` to [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) line
    pub fn to_source(&self) -> String {
        let mut s = String::with_capacity(
            TAG.len() + self.url.len() + self.alt.as_ref().map_or(0, |a| a.len()) + 2,
        );
        s.push_str(TAG);
        s.push(S);
        s.push_str(self.url.trim());
        if let Some(ref alt) = self.alt {
            s.push(S);
            s.push_str(alt.trim());
        }
        s
    }

    // Getters

    /// Get valid [DateTime](https://docs.gtk.org/glib/struct.DateTime.html) for `Self`
    pub fn time(&self, timezone: Option<&TimeZone>) -> Option<DateTime> {
        let a = self.alt.as_ref()?;
        let t = &a[..a.find(S).unwrap_or(a.len())];
        DateTime::from_iso8601(&format!("{t}T00:00:00"), timezone).ok()
    }

    /// Get valid [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `Self`
    pub fn uri(&self, base: Option<&Uri>) -> Option<Uri> {
        // Relative scheme patch
        // https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
        let unresolved_address = match self.url.strip_prefix("//") {
            Some(p) => {
                let b = base?;
                let s = p.trim_start_matches(":");
                &format!(
                    "{}://{}",
                    b.scheme(),
                    if s.is_empty() {
                        format!("{}/", b.host()?)
                    } else {
                        s.into()
                    }
                )
            }
            None => &self.url,
        };
        // Convert address to the valid URI,
        // resolve to absolute URL format if the target is relative
        match base {
            Some(base_uri) => match Uri::resolve_relative(
                Some(&base_uri.to_str()),
                unresolved_address,
                UriFlags::NONE,
            ) {
                Ok(resolved_str) => Uri::parse(&resolved_str, UriFlags::NONE).ok(),
                Err(_) => None,
            },
            None => Uri::parse(unresolved_address, UriFlags::NONE).ok(),
        }
    }
}

#[test]
fn test() {
    use crate::line::Link;

    const SOURCE: &str = "=> gemini://geminiprotocol.net 1965-01-19 Gemini";

    let link = Link::parse(SOURCE).unwrap();

    assert_eq!(link.alt, Some("1965-01-19 Gemini".to_string()));
    assert_eq!(link.url, "gemini://geminiprotocol.net");

    let uri = link.uri(None).unwrap();
    assert_eq!(uri.scheme(), "gemini");
    assert_eq!(uri.host().unwrap(), "geminiprotocol.net");

    let time = link.time(Some(&glib::TimeZone::local())).unwrap();
    assert_eq!(time.year(), 1965);
    assert_eq!(time.month(), 1);
    assert_eq!(time.day_of_month(), 19);

    assert_eq!(link.to_source(), SOURCE);
}

#[test]
fn test_tab() {
    use crate::line::Link;

    const SOURCE: &str = "=> gemlog/\tMy gemlog - verbose ramblings";

    let link = Link::parse(SOURCE).unwrap();
    assert_eq!(link.alt, Some("My gemlog - verbose ramblings".to_string()));
    assert_eq!(link.url, "gemlog/");
}

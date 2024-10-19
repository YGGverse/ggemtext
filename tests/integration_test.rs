use ggemtext::line::{
    code::{inline::Inline, multiline::Multiline, Code},
    header::{Header, Level},
    link::Link,
    list::List,
    quote::Quote,
};

use gtk::glib::{TimeZone, Uri, UriFlags};
use std::fs::read_to_string;

#[test]
fn gemtext() {
    match read_to_string("tests/integration_test.gmi") {
        Ok(gemtext) => {
            // Init tags collection
            let mut code_inline: Vec<Inline> = Vec::new();
            let mut code_multiline: Vec<Multiline> = Vec::new();
            let mut header: Vec<Header> = Vec::new();
            let mut link: Vec<Link> = Vec::new();
            let mut list: Vec<List> = Vec::new();
            let mut quote: Vec<Quote> = Vec::new();

            // Define preformatted buffer
            let mut code_multiline_buffer: Option<Multiline> = None;

            // Define base URI as integration_test.gmi contain one relative link
            let base = match Uri::parse("gemini://geminiprotocol.net", UriFlags::NONE) {
                Ok(uri) => Some(uri),
                Err(_) => None,
            };

            // Define timezone as integration_test.gmi contain one links with date
            let timezone = Some(TimeZone::local());

            // Parse document by line
            for line in gemtext.lines() {
                // Inline code
                if let Some(result) = Code::inline_from(line) {
                    code_inline.push(result);
                }

                // Multiline code
                match code_multiline_buffer {
                    None => {
                        if let Some(code) = Code::multiline_begin_from(line) {
                            code_multiline_buffer = Some(code);
                        }
                    }
                    Some(ref mut result) => {
                        Code::multiline_continue_from(result, line);
                        if result.completed {
                            code_multiline.push(code_multiline_buffer.take().unwrap());
                            code_multiline_buffer = None;
                        }
                    }
                };

                // Header
                if let Some(result) = Header::from(line) {
                    header.push(result);
                }

                // Link
                if let Some(result) = Link::from(line, base.as_ref(), timezone.as_ref()) {
                    link.push(result);
                }

                // List
                if let Some(result) = List::from(line) {
                    list.push(result);
                }

                // Quote
                if let Some(result) = Quote::from(line) {
                    quote.push(result);
                }
            }

            // Assert tags quantity
            assert_eq!(code_inline.len(), 1);
            assert_eq!(code_multiline.len(), 2);
            assert_eq!(header.len(), 3);
            assert_eq!(link.len(), 5);
            assert_eq!(list.len(), 3);
            assert_eq!(quote.len(), 1);
        }
        // Could not load gemtext file
        Err(_) => {
            assert!(false);
        }
    }
}

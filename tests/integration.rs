use ggemtext::line::{
    code::{Inline, Multiline},
    header::{Header, Level},
    link::Link,
    list::List,
    quote::Quote,
};

use glib::{TimeZone, Uri, UriFlags};
use std::fs;

#[test]
fn gemtext() {
    match fs::read_to_string("tests/integration.gmi") {
        Ok(gemtext) => {
            // Init tags collection
            let mut code_inline: Vec<Inline> = Vec::new();
            let mut code_multiline: Vec<Multiline> = Vec::new();
            let mut headers: Vec<Header> = Vec::new();
            let mut links: Vec<Link> = Vec::new();
            let mut list: Vec<List> = Vec::new();
            let mut quote: Vec<Quote> = Vec::new();

            // Define preformatted buffer
            let mut code_multiline_buffer: Option<Multiline> = None;

            // Define base URI as integration.gmi contain one relative link
            let base = Uri::parse("gemini://geminiprotocol.net", UriFlags::NONE).unwrap();

            // Define timezone as integration.gmi contain one links with date
            let timezone = TimeZone::local();

            // Parse document by line
            for line in gemtext.lines() {
                // Inline code
                if let Some(result) = Inline::parse(line) {
                    code_inline.push(result);
                    continue;
                }

                // Multiline code
                match code_multiline_buffer {
                    None => {
                        if let Some(code) = Multiline::begin_from(line) {
                            code_multiline_buffer = Some(code);
                            continue;
                        }
                    }
                    Some(ref mut result) => {
                        assert!(Multiline::continue_from(result, line).is_ok());
                        if result.completed {
                            code_multiline.push(code_multiline_buffer.take().unwrap());
                            code_multiline_buffer = None;
                        }
                        continue;
                    }
                };

                // Header
                if let Some(result) = Header::parse(line) {
                    headers.push(result);
                    continue;
                }

                // Link
                if let Some(result) = Link::parse(line) {
                    links.push(result);
                    continue;
                }

                // List
                if let Some(result) = List::parse(line) {
                    list.push(result);
                    continue;
                }

                // Quote
                if let Some(result) = Quote::parse(line) {
                    quote.push(result);
                    continue;
                }
            }

            // Validate inline code
            assert_eq!(code_inline.len(), 1);
            assert_eq!(code_inline.first().unwrap().value, "inline code");

            // Validate multiline code
            assert_eq!(code_multiline.len(), 2);

            {
                let item = code_multiline.first().unwrap();
                assert_eq!(item.alt.clone().unwrap(), "alt text");

                assert_eq!(item.value.lines().count(), 2);

                let mut lines = item.value.lines();
                assert_eq!(lines.next().unwrap(), "multi");
                assert_eq!(lines.next().unwrap(), "    preformatted line");
            } // #1

            {
                let item = code_multiline.get(1).unwrap();
                assert_eq!(item.alt.clone(), None);

                assert_eq!(item.value.lines().count(), 2);

                let mut lines = item.value.lines();
                assert_eq!(lines.next().unwrap(), "alt-less");
                assert_eq!(lines.next().unwrap(), "    preformatted line");
            } // #2

            // Validate headers
            assert_eq!(headers.len(), 3);

            fn to_u8(level: &Level) -> u8 {
                match level {
                    Level::H1 => 1,
                    Level::H2 => 2,
                    Level::H3 => 3,
                }
            } // comparison helper
            let mut header = headers.iter();
            {
                let item = header.next().unwrap();

                assert_eq!(to_u8(&item.level), to_u8(&Level::H1));
                assert_eq!(item.value, "H1");
            } // #1
            {
                let item = header.next().unwrap();

                assert_eq!(to_u8(&item.level), to_u8(&Level::H2));
                assert_eq!(item.value, "H2");
            } // #2
            {
                let item = header.next().unwrap();

                assert_eq!(to_u8(&item.level), to_u8(&Level::H3));
                assert_eq!(item.value, "H3");
            } // #3

            // Validate links
            assert_eq!(links.len(), 9);
            let mut link = links.iter();
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, None);
                assert_eq!(item.time(Some(&timezone)), None);
                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_str(),
                    "gemini://geminiprotocol.net"
                );
            } // #1
            {
                let item = link.next().unwrap();

                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net"
                );

                let time = item.time(Some(&timezone)).unwrap();
                assert_eq!(time.year(), 1965);
                assert_eq!(time.month(), 1);
                assert_eq!(time.day_of_month(), 19);

                assert_eq!(item.alt, Some("1965-01-19".to_string()));
            } // #2
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt.clone().unwrap(), "Gemini");
                assert_eq!(item.time(Some(&timezone)), None);
                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net"
                );
            } // #3
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, Some("1965-01-19 Gemini".to_string()));

                let time = item.time(Some(&timezone)).unwrap();
                assert_eq!(time.year(), 1965);
                assert_eq!(time.month(), 1);
                assert_eq!(time.day_of_month(), 19);

                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net"
                );
            } // #4
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, Some("1965-01-19 Gemini".to_string()));

                let time = item.time(Some(&timezone)).unwrap();
                assert_eq!(time.year(), 1965);
                assert_eq!(time.month(), 1);
                assert_eq!(time.day_of_month(), 19);

                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net/docs/gemtext.gmi"
                );
            } // #5
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, None);
                assert_eq!(item.time(Some(&timezone)), None);
                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net"
                );
            } // #6
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, None);
                assert_eq!(item.time(Some(&timezone)), None);
                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net"
                );
            } // #7
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, None);
                assert_eq!(item.time(Some(&timezone)), None);
                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net/path"
                );
            } // #8
            {
                let item = link.next().unwrap();

                assert_eq!(item.alt, None);
                assert_eq!(item.time(Some(&timezone)), None);
                assert_eq!(
                    item.uri(Some(&base)).unwrap().to_string(),
                    "gemini://geminiprotocol.net/"
                );
            } // #9

            // Validate lists
            assert_eq!(list.len(), 2);
            assert_eq!(list.first().unwrap().value, "Listing item 1");
            assert_eq!(list.last().unwrap().value, "Listing item 2");

            // Validate quotes
            assert_eq!(quote.len(), 1);
            assert_eq!(quote.first().unwrap().value, "quoted string");
        }
        // Could not load gemtext file
        Err(_) => panic!(),
    }
}

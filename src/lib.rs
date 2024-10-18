mod line;

#[cfg(test)]
mod tests {
    use super::line::{
        code::Code,
        header::{Header, Level},
        link::Link,
        list::List,
        quote::Quote,
    };

    #[test]
    fn line() {
        // Code
        match Code::inline_from("```inline```") {
            Some(inline) => {
                assert_eq!(inline.value, "inline");
            }
            None => assert!(false),
        };

        match Code::multiline_begin_from("```alt") {
            Some(mut multiline) => {
                Code::multiline_continue_from(&mut multiline, "line 1");
                Code::multiline_continue_from(&mut multiline, "line 2");
                Code::multiline_continue_from(&mut multiline, "```");

                assert!(multiline.completed);
                assert_eq!(multiline.alt, Some("alt".into()));
                assert_eq!(multiline.buffer.len(), 3);
            }
            None => assert!(false),
        };

        // Header
        match Header::from("# H1") {
            Some(h1) => {
                assert_eq!(h1.level as i8, Level::H1 as i8);
                assert_eq!(h1.value, "H1");
            }
            None => assert!(false),
        };

        match Header::from("## H2") {
            Some(h1) => {
                assert_eq!(h1.level as i8, Level::H2 as i8);
                assert_eq!(h1.value, "H2");
            }
            None => assert!(false),
        };

        match Header::from("### H3") {
            Some(h1) => {
                assert_eq!(h1.level as i8, Level::H3 as i8);
                assert_eq!(h1.value, "H3");
            }
            None => assert!(false),
        };

        // Link
        match Link::from("=> gemini://geminiprotocol.net Gemini", None, None) {
            Some(link) => {
                assert_eq!(link.alt, Some("Gemini".into()));
                assert_eq!(link.uri.to_string(), "gemini://geminiprotocol.net");
                // @TODO timestamp
            }
            None => assert!(false),
        }; // @TODO options

        // List
        match List::from("* Item") {
            Some(list) => {
                assert_eq!(list.value, "Item");
            }
            None => assert!(false),
        };

        // Quote
        match Quote::from("> Quote") {
            Some(quote) => {
                assert_eq!(quote.value, "Quote");
            }
            None => assert!(false),
        };
    }
}

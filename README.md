# ggemtext

![Build](https://github.com/YGGverse/ggemtext/actions/workflows/build.yml/badge.svg)
[![Documentation](https://docs.rs/ggemtext/badge.svg)](https://docs.rs/ggemtext)
[![crates.io](https://img.shields.io/crates/v/ggemtext.svg)](https://crates.io/crates/ggemtext)

Glib-oriented [Gemtext](https://geminiprotocol.net/docs/gemtext.gmi) API

## Install

``` bash
cargo add ggemtext
```

## Usage

* [Documentation](https://docs.rs/ggemtext/latest/)

### Line

Line parser, useful for [TextTag](https://docs.gtk.org/gtk4/class.TextTag.html) operations in [TextBuffer](https://docs.gtk.org/gtk4/class.TextBuffer.html) context.

Iterate Gemtext lines to continue with [Line](#Line) API:

``` rust
for line in gemtext.lines() {
    // ..
}
```

#### Inline code

**Struct**

``` rust
use ggemtext::line::code::Inline;
match Inline::from("```inline```") {
    Some(inline) => assert_eq!(inline.value, "inline"),
    None => assert!(false),
}
```

**Trait**

``` rust
use ggemtext::line::code::inline::Gemtext;
assert_eq!("```inline```".as_value(), Some("inline"))
assert_eq!("inline".to_source(), "```inline```")
```

#### Multiline code

``` rust
use ggemtext::line::code::Multiline;
match Multiline::begin_from("```alt") {
    Some(mut multiline) => {
        assert!(Multiline::continue_from(&mut multiline, "line 1").is_ok());
        assert!(Multiline::continue_from(&mut multiline, "line 2").is_ok());
        assert!(Multiline::continue_from(&mut multiline, "```").is_ok()); // complete

        assert!(multiline.completed);
        assert_eq!(multiline.alt, Some("alt".into()));
        assert_eq!(multiline.buffer.len(), 3);
    }
    None => assert!(false),
}
```

#### Header

**Struct**

``` rust
use ggemtext::line::{Header, header::Level};
match Header::parse("# H1") {
    Some(h1) => {
        assert_eq!(h1.level as u8, Level::H1 as u8);
        assert_eq!(h1.value, "H1");
    }
    None => assert!(false),
} // H1, H2, H3
```

**Trait**

``` rust
use ggemtext::line::header::{Gemtext, Level};
assert_eq!("# H1".as_value(), Some("H1"));
assert_eq!("H1".to_source(&Level::H1), "# H1");
// H1, H2, H3
```

#### Link

``` rust
use ggemtext::line::Link;
match Link::from(
    "=> gemini://geminiprotocol.net 1965-01-19 Gemini",
    None, // absolute path given, base not wanted
    Some(&glib::TimeZone::local()),
) {
    Some(link) => {
        // Alt
        assert_eq!(link.alt, Some("Gemini".into()));

        // Date
        match link.timestamp {
            Some(timestamp) => {
                assert_eq!(timestamp.year(), 1965);
                assert_eq!(timestamp.month(), 1);
                assert_eq!(timestamp.day_of_month(), 19);
            }
            None => assert!(false),
        }

        // URI
        assert_eq!(link.uri.to_string(), "gemini://geminiprotocol.net");
    }
    None => assert!(false),
}
```

#### List

**Struct**

``` rust
use ggemtext::line::List;
match List::parse("* Item") {
    Some(list) => assert_eq!(list.value, "Item"),
    None => assert!(false),
}
```

**Trait**

``` rust
use ggemtext::line::list::Gemtext;
assert_eq!("* Item".as_value(), Some("Item"))
assert_eq!("Item".to_source(), "* Item")
```

#### Quote

**Struct**

``` rust
use ggemtext::line::Quote;
match Quote::parse("> Quote") {
    Some(quote) => assert_eq!(quote.value, "Quote"),
    None => assert!(false),
}
```

**Trait**

``` rust
use ggemtext::line::quote::Gemtext;
assert_eq!("> Quote".as_value(), Some("Quote"))
assert_eq!("Quote".to_source(), "> Quote")
```

## Integrations

* [Yoda](https://github.com/YGGverse/Yoda) - Browser for Gemini Protocol

## See also

* [ggemini](https://github.com/YGGverse/ggemini) - Glib-oriented client for Gemini Protocol
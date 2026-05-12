# ggemtext

![build](https://github.com/YGGverse/ggemtext/actions/workflows/build.yml/badge.svg)
[![dependencies](https://deps.rs/repo/github/YGGverse/ggemtext/status.svg)](https://deps.rs/repo/github/YGGverse/ggemtext)
[![documentation](https://docs.rs/ggemtext/badge.svg)](https://docs.rs/ggemtext)
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

#### Code

``` rust
use ggemtext::line::Code;
match Code::begin_from("```alt") {
    Some(mut code) => {
        assert!(code.continue_from("line 1").is_ok());
        assert!(code.continue_from("line 2").is_ok());
        assert!(code.continue_from("```").is_ok()); // complete

        assert!(code.is_completed);
        assert_eq!(code.alt, Some("alt".into()));
        assert_eq!(code.value.len(), 12 + 2); // +NL
    }
    None => unreachable!(),
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
    None => unreachable!(),
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
```

#### List

**Struct**

``` rust
use ggemtext::line::List;
match List::parse("* Item") {
    Some(list) => assert_eq!(list.value, "Item"),
    None => unreachable!(),
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
    None => unreachable!(),
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
mod line;

#[cfg(test)]
mod tests {
    use super::line::header::{Header, Level};

    #[test]
    fn h1() {
        match Header::from("# H1") {
            Some(h1) => {
                assert_eq!(h1.level as i32, Level::H1 as i32); // @TODO
                assert_eq!(h1.value, "H1");
            }
            None => assert!(false),
        };
    }

    // @TODO other tags
}

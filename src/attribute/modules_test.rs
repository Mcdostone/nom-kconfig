use crate::{assert_parsing_eq, attribute::parse_modules};

#[test]
fn test_parse_modules() {
    assert_parsing_eq!(parse_modules, "modules", Ok(("", ())))
}

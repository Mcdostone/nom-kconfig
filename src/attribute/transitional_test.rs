use crate::{assert_parsing_eq, attribute::transitional::parse_transitional};

#[test]
fn test_parse_transitional() {
    assert_parsing_eq!(parse_transitional, "transitional", Ok(("", ())))
}

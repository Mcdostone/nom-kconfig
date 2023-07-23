use crate::{assert_parsing_eq, attribute::parse_optional};

#[test]
fn test_parse_optional() {
    assert_parsing_eq!(parse_optional, "optional", Ok(("", ())))
}

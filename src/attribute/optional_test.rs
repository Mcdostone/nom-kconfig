use crate::{assert_parsing_eq, attribute::optional::parse_optional};

#[test]
fn test_parse_optional() {
    let input = "optional";
    assert_parsing_eq!(parse_optional, input, Ok(("", ())))
}

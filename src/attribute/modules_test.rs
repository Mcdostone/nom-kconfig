use crate::{assert_parsing_eq, attribute::modules::parse_modules};

#[test]
fn test_parse_modules() {
    let input = "modules";
    assert_parsing_eq!(parse_modules, input, Ok(("", ())))
}

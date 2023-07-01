use crate::{assert_parsing_eq, attribute::help::parse_help};

#[test]
fn test_parse_help() {
    let input = "help\n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}

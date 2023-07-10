use crate::{assert_parsing_eq, attribute::help::parse_help};

#[test]
fn test_parse_help() {
    let input = "help\n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}

#[test]
fn test_parse_help_space() {
    let input = "help   \n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}

// 3.2/drivers/net/ethernet/stmicro/stmmac/Kconfig
#[test]
fn test_parse_help_prefixed_by_hypen() {
    let input = "-- help\n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}
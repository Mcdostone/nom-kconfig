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

// 2.5.45/drivers/mtd/maps/Kconfig
#[test]
fn test_parse_help_encoding() {
    let input = "-- help\n Mapping for the Flaga digital module. If you don�t have one, ignore this setting.";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "Mapping for the Flaga digital module. If you don�t have one, ignore this setting."
                .to_string()
        ))
    )
}

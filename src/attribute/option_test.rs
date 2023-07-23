use crate::{
    assert_parsing_eq,
    attribute::{parse_option, OptionValues},
};

#[test]
fn test_parse_option_defconfig_list() {
    let input = "option defconfig_list";
    assert_parsing_eq!(parse_option, input, Ok(("", OptionValues::DefconfigList)))
}

#[test]
fn test_parse_option_modules() {
    let input = "option modules";
    assert_parsing_eq!(parse_option, input, Ok(("", OptionValues::Modules)))
}

#[test]
fn test_parse_option_env() {
    assert_parsing_eq!(
        parse_option,
        r#"option env ="SHELL""#,
        Ok(("", OptionValues::Env("SHELL".to_string())))
    )
}

#[test]
fn test_parse_option_allnoconfig_y() {
    let input = "option  allnoconfig_y";
    assert_parsing_eq!(parse_option, input, Ok(("", OptionValues::AllNoConfigY)))
}

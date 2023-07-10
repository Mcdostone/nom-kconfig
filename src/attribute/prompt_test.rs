use crate::{assert_parsing_eq, attribute::prompt::parse_prompt_option};

#[test]
fn test_parse_prompt() {
    let input = "\"scripts/Kconfig.include\"";
    assert_parsing_eq!(
        parse_prompt_option,
        input,
        Ok(("", "scripts/Kconfig.include"))
    )
}

#[test]
fn test_parse_prompt_1() {
    let input = "\"Support in-kernel module decompression\"";
    assert_parsing_eq!(
        parse_prompt_option,
        input,
        Ok(("", "Support in-kernel module decompression"))
    )
}

// 3.0.18/arch/arm/plat-tcc/Kconfig
#[test]
fn test_parse_prompt_no_quote() {
    let input = " TCC8000";
    assert_parsing_eq!(parse_prompt_option, input, Ok(("", "TCC8000")))
}

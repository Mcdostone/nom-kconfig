use crate::{
    assert_parsing_eq,
    symbol::{
        parse_constant_bool, parse_constant_hex, parse_constant_int, parse_constant_string,
        parse_constant_tristate, parse_symbol, ConstantSymbol, Symbol,
    },
    tristate::Tristate,
};

#[test]
fn test_parse_symbol() {
    let input = "\"hello\"";
    assert_parsing_eq!(
        parse_symbol,
        input,
        Ok((
            "",
            Symbol::Constant(ConstantSymbol::String("hello".to_string()))
        ))
    )
}

#[test]
fn test_parse_non_constant_symbol() {
    let input = " ALPHA_MIATA";
    assert_parsing_eq!(
        parse_symbol,
        input,
        Ok(("", Symbol::NonConstant(input.trim().to_string())))
    );

    // Symbol with a hyphen: https://github.com/openwrt/openwrt/blob/75f2f960caa03dd2d1aa1bc38de908bb5a47c979/config/Config-build.in#L72
    let input = " PACKAGE_apk-mbedtls";
    assert_parsing_eq!(
        parse_symbol,
        input,
        Ok(("", Symbol::NonConstant(input.trim().to_string())))
    )
}

#[test]
fn test_symbol_to_string() {
    assert_eq!(Symbol::NonConstant("KVM".to_string()).to_string(), "KVM");
}

#[test]
fn test_parse_constant_bool() {
    assert_parsing_eq!(
        parse_constant_bool,
        "y",
        Ok(("", ConstantSymbol::Boolean(true)))
    );

    assert_parsing_eq!(
        parse_constant_bool,
        "n",
        Ok(("", ConstantSymbol::Boolean(false)))
    );
}

#[test]
fn test_parse_constant_tristate() {
    assert_parsing_eq!(
        parse_constant_tristate,
        "m",
        Ok(("", ConstantSymbol::Tristate(Tristate::Module)))
    );

    assert_parsing_eq!(
        parse_constant_tristate,
        "y",
        Ok(("", ConstantSymbol::Tristate(Tristate::Yes)))
    );

    assert_parsing_eq!(
        parse_constant_tristate,
        "n",
        Ok(("", ConstantSymbol::Tristate(Tristate::No)))
    );
}

#[test]
fn test_parse_constant_int() {
    assert_parsing_eq!(
        parse_constant_int,
        "314",
        Ok(("", ConstantSymbol::Integer(314)))
    );
}

#[test]
fn test_parse_constant_hex() {
    assert_parsing_eq!(
        parse_constant_hex,
        "0x314",
        Ok(("", ConstantSymbol::Hex("0x314".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_hex,
        "0XAE72",
        Ok(("", ConstantSymbol::Hex("0XAE72".to_string())))
    );
}

#[test]
fn test_parse_constant_string() {
    assert_parsing_eq!(
        parse_constant_string,
        "'hello'",
        Ok(("", ConstantSymbol::String("hello".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_string,
        "\"world\"",
        Ok(("", ConstantSymbol::String("world".to_string())))
    );
}

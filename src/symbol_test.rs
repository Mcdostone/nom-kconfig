use crate::{
    assert_parsing_eq,
    symbol::{
        parse_constant_bool, parse_constant_hex, parse_constant_int, parse_constant_string,
        parse_constant_symbol, parse_constant_tristate, parse_symbol, Symbol,
    },
};

#[test]
fn test_parse_symbol() {
    let input = "\"hello\"";
    assert_parsing_eq!(
        parse_symbol,
        input,
        Ok(("", Symbol::Constant("hello".to_string())))
    )
}

#[test]
fn test_parse_non_constant_symbol() {
    let input = " ALPHA_MIATA";
    assert_parsing_eq!(
        parse_symbol,
        input,
        Ok(("", Symbol::NonConstant("ALPHA_MIATA".to_string())))
    )
}

#[test]
fn test_parse_constant_symbol() {
    assert_parsing_eq!(
        parse_constant_symbol,
        "0",
        Ok(("", Symbol::Constant("0".to_string())))
    )
}

#[test]
fn test_symbol_to_string() {
    assert_eq!(Symbol::Constant("KVM".to_string()).to_string(), "KVM");
    assert_eq!(Symbol::NonConstant("KVM".to_string()).to_string(), "KVM");
}

#[test]
fn test_parse_constant_bool() {
    assert_parsing_eq!(
        parse_constant_bool,
        "y",
        Ok(("", Symbol::Constant("y".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_bool,
        "n",
        Ok(("", Symbol::Constant("n".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_bool,
        "'y'",
        Ok(("", Symbol::Constant("y".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_bool,
        "'n'",
        Ok(("", Symbol::Constant("n".to_string())))
    );
}

#[test]
fn test_parse_constant_tristate() {
    assert_parsing_eq!(
        parse_constant_tristate,
        "m",
        Ok(("", Symbol::Constant("m".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_tristate,
        "'m'",
        Ok(("", Symbol::Constant("m".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_tristate,
        "\"m\"",
        Ok(("", Symbol::Constant("m".to_string())))
    );
}

#[test]
fn test_parse_constant_int() {
    assert_parsing_eq!(
        parse_constant_int,
        "314",
        Ok(("", Symbol::Constant("314".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_int,
        "'89456'",
        Ok(("", Symbol::Constant("89456".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_int,
        "\"67\"",
        Ok(("", Symbol::Constant("67".to_string())))
    );
}

#[test]
fn test_parse_constant_hex() {
    assert_parsing_eq!(
        parse_constant_hex,
        "0x314",
        Ok(("", Symbol::Constant("0x314".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_hex,
        "0XAE72",
        Ok(("", Symbol::Constant("0XAE72".to_string())))
    );
}

#[test]
fn test_parse_constant_string() {
    assert_parsing_eq!(
        parse_constant_string,
        "'hello'",
        Ok(("", Symbol::Constant("hello".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_string,
        "\"world\"",
        Ok(("", Symbol::Constant("world".to_string())))
    );
}

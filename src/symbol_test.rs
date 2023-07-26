use crate::{
    assert_parsing_eq,
    symbol::{parse_symbol, Symbol},
};

#[test]
fn test_parse_symbol() {
    let input = "\"hello\"";
    assert_parsing_eq!(
        parse_symbol,
        input,
        Ok(("", Symbol::NonConstant("\"hello\"".to_string())))
    )
}

#[test]
fn test_symbol_to_string() {
    assert_eq!(Symbol::Constant("KVM".to_string()).to_string(), "KVM");
    assert_eq!(
        Symbol::NonConstant("KVM".to_string()).to_string(),
        r#""KVM""#
    );
}

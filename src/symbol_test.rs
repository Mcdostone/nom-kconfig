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

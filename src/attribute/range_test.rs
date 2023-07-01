use crate::{
    assert_parsing_eq,
    attribute::range::{parse_range, Range},
    symbol::Symbol,
};

#[test]
fn test_parse_range() {
    let input = "range 1 5";
    assert_parsing_eq!(
        parse_range,
        input,
        Ok((
            "",
            Range {
                lhs: Symbol::Constant("1".to_string()),
                rhs: Symbol::Constant("5".to_string()),
                r#if: None
            }
        ))
    )
}

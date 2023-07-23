use crate::{
    assert_parsing_eq,
    attribute::{parse_range, Range},
    symbol::Symbol,
};

#[test]
fn test_parse_range() {
    assert_parsing_eq!(
        parse_range,
        "range 1 5",
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

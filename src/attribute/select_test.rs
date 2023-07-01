use crate::{
    assert_parsing_eq,
    attribute::select::{parse_select, Select},
    symbol::Symbol,
};

#[test]
fn test_parse_select() {
    let input = "select MTK_INFRACFG";
    assert_parsing_eq!(
        parse_select,
        input,
        Ok((
            "",
            Select {
                r#if: None,
                symbol: Symbol::Constant("MTK_INFRACFG".to_string())
            }
        ))
    )
}

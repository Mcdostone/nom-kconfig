use crate::{
    assert_parsing_eq,
    attribute::{parse_select, Select},
};

#[test]
fn test_parse_select() {
    assert_parsing_eq!(
        parse_select,
        "select MTK_INFRACFG",
        Ok((
            "",
            Select {
                r#if: None,
                symbol: "MTK_INFRACFG".to_string()
            }
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::{parse_enable, Enable},
};

#[test]
fn test_parse_enable() {
    assert_parsing_eq!(
        parse_enable,
        "enable MTK_INFRACFG",
        Ok((
            "",
            Enable {
                symbol: "MTK_INFRACFG".to_string()
            }
        ))
    )
}

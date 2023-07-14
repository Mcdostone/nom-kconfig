use crate::{
    assert_parsing_eq,
    attribute::{enable::{Enable, parse_enable}},
};

#[test]
fn test_parse_enable() {
    let input = "enable MTK_INFRACFG";
    assert_parsing_eq!(
        parse_enable,
        input,
        Ok((
            "",
            Enable {
                symbol: "MTK_INFRACFG".to_string()
            }
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::{parse_enable, Enable},
};

// v2.6.1/drivers/net/wireless/Kconfig
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

use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{EntryType, Type},
        Attribute,
    },
    entry::config::{parse_config, Config},
};

#[test]
fn test_parse_config() {
    let input = "config KVM hex \"wow\"";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            Config {
                symbol: "KVM".to_string(),
                attributes: vec!(Attribute::Type(EntryType {
                    r#type: Type::Hex,
                    prompt: Some("wow".to_string()),
                    r#if: None
                }))
            }
        ))
    )
}

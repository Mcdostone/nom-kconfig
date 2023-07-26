use crate::{
    assert_parsing_eq, assert_parsing_fail,
    attribute::r#type::{ConfigType, Type},
    entry::{config::parse_tristate_config, parse_config, Config},
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
                r#type: ConfigType {
                    r#type: Type::Hex,
                    prompt: Some("wow".to_string()),
                    r#if: None
                },
                attributes: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_config_bool() {
    let input = "config KVM\n    bool";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            Config {
                symbol: "KVM".to_string(),
                r#type: ConfigType {
                    r#type: Type::Bool,
                    prompt: None,
                    r#if: None
                },
                attributes: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_config_tristate() {
    let input = "config RAPIDIO_ENUM_BASIC\n    tristate";
    assert_parsing_eq!(
        parse_tristate_config,
        input,
        Ok((
            "",
            Config {
                symbol: "RAPIDIO_ENUM_BASIC".to_string(),
                r#type: ConfigType {
                    r#type: Type::Tristate,
                    prompt: None,
                    r#if: None
                },
                attributes: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_config_type_required() {
    let input = "config RAPIDIO_ENUM_BASIC 	select HAVE_KVM_IRQCHIP";
    assert_parsing_fail!(parse_config, input)
}

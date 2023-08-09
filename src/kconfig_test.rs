use crate::{
    assert_parsing_eq,
    attribute::r#type::{ConfigType, Type},
    entry::{config::Config, Choice},
    kconfig::parse_kconfig,
    Attribute, Entry, Kconfig,
};

#[test]
fn test_parse_kconfig() {
    let input = "
    config SND_INTEL_NHLT
        tristate
        # this config should be selected only for Intel ACPI platforms.
        # A fallback is provided so that the code compiles in all cases.";
    assert_parsing_eq!(
        parse_kconfig,
        input,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(Entry::Config(Config {
                    symbol: "SND_INTEL_NHLT".to_string(),
                    attributes: vec!(Attribute::Type(ConfigType {
                        r#type: Type::Tristate(None),
                        r#if: None
                    }))
                }))
            }
        ))
    )
}

#[test]
fn test_parse_kconfig_choice() {
    let input = "
choice
config RAPIDIO_ENUM_BASIC
	tristate

endchoice
";
    assert_parsing_eq!(
        parse_kconfig,
        input,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(Entry::Choice(Choice {
                    options: vec!(),
                    entries: vec!(Entry::Config(Config {
                        symbol: "RAPIDIO_ENUM_BASIC".to_string(),
                        attributes: vec!(Attribute::Type(ConfigType {
                            r#type: Type::Tristate(None),
                            r#if: None
                        }))
                    }))
                })),
            }
        ))
    )
}

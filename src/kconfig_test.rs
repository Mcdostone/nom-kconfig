use crate::{
    assert_parsing_eq,
    attribute::r#type::{EntryType, Type},
    entry::config::Config,
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
                    attributes: vec!(Attribute::Type(EntryType {
                        r#type: Type::Tristate,
                        prompt: None,
                        r#if: None
                    }))
                }))
            }
        ))
    )
}

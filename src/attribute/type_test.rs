use crate::{
    assert_parsing_eq,
    attribute::r#type::{parse_type, EntryType, Type},
};

#[test]
fn test_parse_type() {
    let input = " string";
    assert_parsing_eq!(
        parse_type,
        input,
        Ok((
            "",
            EntryType {
                r#type: Type::String,
                prompt: None,
                r#if: None
            },
        ))
    )
}

// 3.0.18/arch/arm/plat-tcc/Kconfig
#[test]
fn test_parse_type_with_weird_prompt() {
    let input = "bool TCC8000";
    assert_parsing_eq!(
        parse_type,
        input,
        Ok((
            "",
            EntryType {
                r#type: Type::Bool,
                prompt: Some("TCC8000".to_string()),
                r#if: None
            },
        ))
    )
}

// 3.0.18/arch/powerpc/kvm/Kconfig
#[test]
fn test_parse_type_bool() {
    let input = "bool";
    assert_parsing_eq!(
        parse_type,
        input,
        Ok((
            "",
            EntryType {
                r#type: Type::Bool,
                prompt: None,
                r#if: None
            },
        ))
    )
}

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

use crate::{
    assert_parsing_eq,
    attribute::r#type::{ConfigType, Type},
    entry::{parse_entries, Comment, Config},
    Attribute, Entry,
};

#[test]
fn test_parse_entries() {
    let input = r#"config KVM
        bool 
        comment "some configs""#;
    assert_parsing_eq!(
        parse_entries,
        input,
        Ok((
            "",
            vec!(
                Entry::Config(Config {
                    symbol: "KVM".to_string(),
                    attributes: vec!(Attribute::Type(ConfigType {
                        r#type: Type::Bool(None),
                        r#if: None
                    }))
                }),
                Entry::Comment(Comment {
                    prompt: "some configs".to_string(),
                    dependencies: vec!()
                }),
            )
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::r#type::{ConfigType, Type},
    entry::{parse_entries, Comment, Config},
    Entry,
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
                    r#type: ConfigType {
                        r#type: Type::Bool,
                        prompt: None,
                        r#if: None
                    },
                    attributes: vec!()
                }),
                Entry::Comment(Comment {
                    prompt: "some configs".to_string(),
                    dependencies: vec!()
                }),
            )
        ))
    )
}

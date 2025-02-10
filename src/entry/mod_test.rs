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

#[test]
fn test_double_indented_entries() {
    let input = r#"mainmenu "MAIN"

    config A
        bool
        help
            - Lorem ipsum dolor sit amet, consetetur sadipscing elitr.
                - Lorem ipsum dolor sit amet, consetetur sadipscing elitr.

    config B
        bool
"#;
    assert_parsing_eq!(
        parse_entries,
        input,
        Ok((
            "",
            vec!(
                Entry::MainMenu(crate::entry::MainMenu {
                    prompt: "MAIN".to_string()
                }),
                Entry::Config(Config {
                    symbol: "A".to_string(),
                    attributes: vec!(
                        Attribute::Type(ConfigType {
                            r#type: Type::Bool(None),
                            r#if: None
                        }),
                        Attribute::Help(
                            "- Lorem ipsum dolor sit amet, consetetur sadipscing elitr.\n    - Lorem ipsum dolor sit amet, consetetur sadipscing elitr.".to_string()
                        )
                    )
                }),
                Entry::Config(Config {
                    symbol: "B".to_string(),
                    attributes: vec![Attribute::Type(ConfigType {
                        r#type: Type::Bool(None),
                        r#if: None
                    })]
                })
            )
        ))
    )
}

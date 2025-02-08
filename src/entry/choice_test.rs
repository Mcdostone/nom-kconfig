use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        AndExpression, Atom, DefaultAttribute, Expression, Term,
    },
    entry::{parse_choice, Choice, Comment, Config},
    symbol::Symbol,
    Attribute, Entry,
};

#[test]
fn test_parse_choice_optional() {
    assert_parsing_eq!(
        parse_choice,
        "choice optional endchoice",
        Ok((
            "",
            Choice {
                options: vec!(Attribute::Optional),
                entries: vec!()
            }
        ))
    )
}

// TODO
// #[test]
// fn test_parse_choice_string_not_allowed() {
//     assert_parsing_fail!(
//         parse_choice,
//         r#"choice
//         config TESTCHOICE1
//     string "Choice 1"
//         endchoice"#
//     )
// }

// 6.4.9/drivers/usb/mtu3/Kconfig
// 6.4.9/drivers/usb/dwc2/Kconfig
#[test]
fn test_parse_choice_with_comment() {
    assert_parsing_eq!(
        parse_choice,
        r#"choice
        bool "MTU3 Mode Selection"
        default USB_MTU3_DUAL_ROLE if (USB && USB_GADGET)

    config USB_MTU3_HOST
        bool "Host only mode"

        comment "Gadget/Dual-role mode requires USB Gadget support to be enabled"

endchoice"#,
        Ok((
            "",
            Choice {
                options: vec!(
                    Attribute::Type(ConfigType {
                        r#type: Type::Bool(Some("MTU3 Mode Selection".to_string())),
                        r#if: None
                    }),
                    Attribute::Default(DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::Constant("USB_MTU3_DUAL_ROLE".to_string()))
                        ))),
                        r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Parenthesis(Box::new(Expression::Term(
                                AndExpression::Expression(vec!(
                                    Term::Atom(Atom::Symbol(Symbol::Constant("USB".to_string()))),
                                    Term::Atom(Atom::Symbol(Symbol::Constant(
                                        "USB_GADGET".to_string()
                                    ))),
                                ))
                            )))
                        ))))
                    })
                ),
                entries: vec!(
                    Entry::Config(Config {
                        symbol: "USB_MTU3_HOST".to_string(),
                        attributes: vec!(Attribute::Type(ConfigType {
                            r#type: Type::Bool(Some("Host only mode".to_string())),
                            r#if: None
                        }))
                    }),
                    Entry::Comment(Comment {
                        prompt: "Gadget/Dual-role mode requires USB Gadget support to be enabled"
                            .to_string(),
                        dependencies: vec!()
                    })
                )
            }
        ))
    )
}

#[test]
fn test_parse_choice_with_symbol() {
    assert_parsing_eq!(
        parse_choice,
        r#"choice DRIVE

            config WARP_DRIVE
                bool "Warp drive"

            config SUBLIGHT_DRIVE
                bool "Sublight drive"
            endchoice"#,
        Ok((
            "",
            Choice {
                options: vec!(),
                entries: vec!(
                    Entry::Config(Config {
                        symbol: "WARP_DRIVE".to_string(),
                        attributes: vec![Attribute::Type(ConfigType {
                            r#type: Type::Bool(Some("Warp drive".to_string())),
                            r#if: None
                        })]
                    }),
                    Entry::Config(Config {
                        symbol: "SUBLIGHT_DRIVE".to_string(),
                        attributes: vec![Attribute::Type(ConfigType {
                            r#type: Type::Bool(Some("Sublight drive".to_string())),
                            r#if: None
                        })]
                    })
                )
            }
        ))
    )
}

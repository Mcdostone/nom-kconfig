#[cfg(feature = "coreboot")]
use crate::attribute::Prompt;
use crate::{
    assert_parsing_eq,
    attribute::r#type::{ConfigType, Type},
    entry::{parse_choice, Choice, Config},
    Attribute, Entry,
};
#[cfg(not(feature = "coreboot"))]
use crate::{
    attribute::{AndExpression, Atom, DefaultAttribute, Expression, Term},
    entry::Comment,
    symbol::Symbol,
};

#[test]
#[cfg(not(feature = "coreboot"))]
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
#[cfg(not(feature = "coreboot"))]
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
                            Atom::Symbol(Symbol::NonConstant("USB_MTU3_DUAL_ROLE".to_string()))
                        ))),
                        r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Parenthesis(Box::new(Expression::Term(
                                AndExpression::Expression(vec!(
                                    Term::Atom(Atom::Symbol(Symbol::NonConstant(
                                        "USB".to_string()
                                    ))),
                                    Term::Atom(Atom::Symbol(Symbol::NonConstant(
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

/// https://github.com/coreboot/coreboot/blob/4e522f49b6944f336aec5048d0fd84832e1b6ff3/src/device/Kconfig#L522
#[test]
#[cfg(feature = "named-choice")]
fn test_parse_named_choice_from_coreboot() {
    assert_parsing_eq!(
        parse_choice,
        r#"choice DEFAULT_SCREEN_ROTATION
	prompt "Default screen orientation"
	
config DEFAULT_SCREEN_ROTATION_NONE
	bool "Non-rotated"
endchoice"#,
        Ok((
            "",
            Choice {
                name: Some("DEFAULT_SCREEN_ROTATION".to_string()),
                options: vec!(Attribute::Prompt(Prompt {
                    prompt: "Default screen orientation".to_string(),
                    r#if: None
                }),),
                entries: vec!(Entry::Config(Config {
                    symbol: "DEFAULT_SCREEN_ROTATION_NONE".to_string(),
                    attributes: vec!(Attribute::Type(ConfigType {
                        r#type: Type::Bool(Some("Non-rotated".to_string())),
                        r#if: None
                    }))
                }))
            }
        ))
    )
}

/// https://github.com/coreboot/coreboot/blob/main/src/Kconfig#L22
#[test]
#[cfg(feature = "named-choice")]
fn test_parse_named_choice_from_coreboot_without_extra_name() {
    assert_parsing_eq!(
        parse_choice,
        r#"choice 
	prompt "CBFS prefix to use"

    config CBFS_PREFIX_FALLBACK
	    bool "fallback"
endchoice"#,
        Ok((
            "",
            Choice {
                name: None,
                options: vec!(Attribute::Prompt(Prompt {
                    prompt: "CBFS prefix to use".to_string(),
                    r#if: None
                }),),
                entries: vec!(Entry::Config(Config {
                    symbol: "CBFS_PREFIX_FALLBACK".to_string(),
                    attributes: vec!(Attribute::Type(ConfigType {
                        r#type: Type::Bool(Some("fallback".to_string())),
                        r#if: None
                    }))
                }))
            }
        ))
    )
}

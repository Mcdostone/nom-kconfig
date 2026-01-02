use crate::attribute::DefaultAttribute;
use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        AndExpression, Atom, CompareExpression, CompareOperator, Expression, Term,
    },
    entry::{config::Config, Choice},
    kconfig::parse_kconfig,
    Attribute, Entry, Kconfig, Symbol,
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

#[test]
/// https://github.com/Mcdostone/nom-kconfig/issues/107#issuecomment-3736187967
fn test_parse_kconfig_bool() {
    let input = r#"
config 64BIT
	bool "64-bit kernel" if "$(SUBARCH)" = "x86"
	default "$(SUBARCH)" != "i386"
"#;
    assert_parsing_eq!(
        parse_kconfig,
        input,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(Entry::Config(Config {
                    symbol: "64BIT".to_string(),
                    attributes: vec!(
                        Attribute::Type(ConfigType {
                            r#type: Type::Bool(Some("64-bit kernel".to_string())),
                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                                Atom::Compare(CompareExpression {
                                    left: Symbol::NonConstant("$(SUBARCH)".to_string()),
                                    operator: CompareOperator::Equal,
                                    right: Symbol::Constant("x86".to_string())
                                })
                            )))),
                        }),
                        Attribute::Default(DefaultAttribute {
                            expression: Expression::Term(AndExpression::Term(Term::Atom(
                                Atom::Compare(CompareExpression {
                                    left: Symbol::NonConstant("$(SUBARCH)".to_string()),
                                    operator: CompareOperator::NotEqual,
                                    right: Symbol::Constant("i386".to_string())
                                })
                            ))),
                            r#if: None
                        })
                    ),
                }))
            }
        ))
    )
}

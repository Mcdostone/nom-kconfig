use std::path::PathBuf;

use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{AndExpression, Atom, Expression, Term},
        r#type::{ConfigType, Type},
        DefaultAttribute, ExpressionToken, FunctionCall, Parameter,
    },
    entry::{config::Config, r#if::If, MenuConfig},
    kconfig::parse_kconfig,
    symbol::Symbol,
    Attribute, Entry, Kconfig, KconfigFile,
};

#[macro_export]
macro_rules! assert_parsing_fail {
    ($fn:ident, $input:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra($input, Default::default()));
        assert!(res.is_err())
    }};
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_parsing_source_eq {
    ($fn:ident, $input:expr, $expected:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra(
            $input,
            KconfigFile {
                ..Default::default()
            },
        ))
        .map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}

// 2.6.25/drivers/ide/Kconfig
#[test]
fn test_parse_type() {
    let input = r#"config BLK_DEV_IDEDMA_SFF
	bool

if PCI

endif"#;
    assert_parsing_eq!(
        parse_kconfig,
        input,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(
                    Entry::Config(Config {
                        symbol: "BLK_DEV_IDEDMA_SFF".to_string(),
                        attributes: vec!(Attribute::Type(ConfigType {
                            r#type: Type::Bool(None),
                            r#if: None
                        }))
                    }),
                    Entry::If(If {
                        condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                            Symbol::NonConstant("PCI".to_string())
                        )))),
                        entries: vec!()
                    })
                )
            },
        ))
    )
}

// 6.4.9/arch/powerpc/platforms/86xx/Kconfig
#[test]
fn test_parse_config_without_attribute() {
    assert_parsing_eq!(
        parse_kconfig,
        r#"config PPC_86xx
        menuconfig PPC_86xx
            bool "86xx-based boards"
        "#,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(
                    Entry::Config(Config {
                        symbol: "PPC_86xx".to_string(),
                        attributes: vec!()
                    }),
                    Entry::MenuConfig(MenuConfig {
                        symbol: "PPC_86xx".to_string(),
                        attributes: vec!(Attribute::Type(ConfigType {
                            r#type: Type::Bool(Some("86xx-based boards".to_string())),
                            r#if: None
                        }))
                    }),
                )
            },
        ))
    )
}

// 4.18/arch/Kconfig
#[test]
fn test_parse_config_string_with_double_quotes() {
    assert_parsing_eq!(
        parse_kconfig,
        r#"
        config PLUGIN_HOSTCC
	string
	default "$(shell,$(srctree)/scripts/gcc-plugin.sh "$(preferred-plugin-hostcc)" "$(HOSTCXX)" "$(CC)")"

menuconfig GCC_PLUGINS
	bool "GCC plugins"
"#,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(
                    Entry::Config(Config {
                        symbol: "PLUGIN_HOSTCC".to_string(),
                        attributes: vec!(
                            Attribute::Type(ConfigType {
                                r#type: Type::String(None),
                                r#if: None
                            }),
                            Attribute::Default(DefaultAttribute {
                                expression: Expression::Term(AndExpression::Term(Term::Atom(
                                    Atom::Function(FunctionCall {
                                        name: "shell".to_string(),
                                        parameters: vec!(Parameter {
                                            tokens: vec!(
                                                ExpressionToken::Function(Box::new(FunctionCall {
                                                    name: "srctree".to_string(),
                                                    parameters: vec![]
                                                })),
                                                ExpressionToken::Literal(
                                                    "/scripts/gcc-plugin.sh".to_string()
                                                ),
                                                ExpressionToken::Space,
                                                ExpressionToken::DoubleQuotes(vec![
                                                    ExpressionToken::Function(Box::new(
                                                        FunctionCall {
                                                            name: "preferred-plugin-hostcc"
                                                                .to_string(),
                                                            parameters: vec![]
                                                        }
                                                    )),
                                                ]),
                                                ExpressionToken::Space,
                                                ExpressionToken::DoubleQuotes(vec![
                                                    ExpressionToken::Variable(
                                                        "HOSTCXX".to_string()
                                                    )
                                                ]),
                                                ExpressionToken::Space,
                                                ExpressionToken::DoubleQuotes(vec![
                                                    ExpressionToken::Variable("CC".to_string())
                                                ])
                                            )
                                        })
                                    })
                                ))),
                                r#if: None
                            })
                        )
                    }),
                    Entry::MenuConfig(MenuConfig {
                        symbol: "GCC_PLUGINS".to_string(),
                        attributes: vec!(Attribute::Type(ConfigType {
                            r#type: Type::Bool(Some("GCC plugins".to_string())),
                            r#if: None
                        }))
                    }),
                )
            },
        ))
    )
}

// https://github.com/Mcdostone/nom-kconfig/issues/57
#[test]
fn test_parse_config_issue_github() {
    assert_parsing_eq!(
        parse_kconfig,
        r#"
config AS_WRUSS
	def_bool $(as-instr64,wrussq %rax$(comma)(%rbx))
	help
	  Supported by binutils
"#,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(Entry::Config(Config {
                    symbol: "AS_WRUSS".to_string(),
                    attributes: vec!(
                        Attribute::Type(ConfigType {
                            r#type: Type::DefBool(Expression::Term(AndExpression::Term(
                                Term::Atom(Atom::Function(FunctionCall {
                                    name: "as-instr64".to_string(),
                                    parameters: vec!(Parameter {
                                        tokens: vec!(
                                            ExpressionToken::Literal("wrussq".to_string()),
                                            ExpressionToken::Space,
                                            ExpressionToken::Literal("%rax".to_string()),
                                            ExpressionToken::Function(Box::new(FunctionCall {
                                                name: "comma".to_string(),
                                                parameters: vec!(),
                                            })),
                                            ExpressionToken::Literal("(%rbx)".to_string()),
                                        )
                                    })
                                }))
                            ))),
                            r#if: None
                        },),
                        Attribute::Help("Supported by binutils".to_string())
                    )
                }),)
            },
        ))
    )
}

#[test]
fn test_set_envs() {
    let mut file = KconfigFile::new(PathBuf::from("."), PathBuf::from("Kconfig"));
    file.set_vars(&[("ARCH", "ARCH_64")]);
    assert_eq!(file.vars.get("ARCH"), Some(&"ARCH_64".to_string()));
}

use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        AndExpression, Atom, DefaultAttribute, Expression, Term,
    },
    entry::{parse_config, Config},
    symbol::Symbol,
    Attribute,
};

#[test]
fn test_parse_config() {
    let input = "config KVM hex \"wow\"";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            Config {
                symbol: "KVM".to_string(),
                attributes: vec!(Attribute::Type(ConfigType {
                    r#type: Type::Hex(Some("wow".to_string())),
                    r#if: None
                }))
            }
        ))
    )
}

// 6.4.9/init/Kconfig
#[test]
fn test_parse_config_no_type() {
    let input = "config ARCH_MMAP_RND_BITS_MIN
	default 18 if 64BIT
	default 8";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            Config {
                symbol: "ARCH_MMAP_RND_BITS_MIN".to_string(),
                attributes: vec!(
                    Attribute::Default(DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Number(18)
                        ))),
                        r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::Constant("64BIT".to_string()))
                        ))))
                    }),
                    Attribute::Default(DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Number(8)
                        ))),
                        r#if: None
                    })
                )
            }
        ))
    )
}

#[test]
fn test_parse_config_tristate() {
    let input = "config RAPIDIO_ENUM_BASIC\n    tristate";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            Config {
                symbol: "RAPIDIO_ENUM_BASIC".to_string(),
                attributes: vec!(Attribute::Type(ConfigType {
                    r#type: Type::Tristate(None),
                    r#if: None
                }))
            }
        ))
    )
}

// 6.4.9/arch/sh/Kconfig
#[test]
fn test_parse_config_def_bool_multiline_expression() {
    let input = "config SH_CLK_CPG_LEGACY
	depends on SH_CLK_CPG
	def_bool y if !CPU_SUBTYPE_SH7785 && !ARCH_SHMOBILE && \
		      !CPU_SHX3 && !CPU_SUBTYPE_SH7757 && \
		      !CPU_SUBTYPE_SH7734 && !CPU_SUBTYPE_SH7264 && \
		      !CPU_SUBTYPE_SH7269";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            Config {
                symbol: "SH_CLK_CPG_LEGACY".to_string(),
                attributes: vec!(
                    Attribute::DependsOn(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Symbol(Symbol::Constant("SH_CLK_CPG".to_string()))
                    )))),
                    Attribute::Type(ConfigType {
                        r#type: Type::DefBool(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::Constant("y".to_string()))
                        )))),
                        r#if: Some(Expression::Term(AndExpression::Expression(vec!(
                            Term::Not(Atom::Symbol(Symbol::Constant(
                                "CPU_SUBTYPE_SH7785".to_string()
                            ))),
                            Term::Not(Atom::Symbol(Symbol::Constant("ARCH_SHMOBILE".to_string()))),
                            Term::Not(Atom::Symbol(Symbol::Constant("CPU_SHX3".to_string()))),
                            Term::Not(Atom::Symbol(Symbol::Constant(
                                "CPU_SUBTYPE_SH7757".to_string()
                            ))),
                            Term::Not(Atom::Symbol(Symbol::Constant(
                                "CPU_SUBTYPE_SH7734".to_string()
                            ))),
                            Term::Not(Atom::Symbol(Symbol::Constant(
                                "CPU_SUBTYPE_SH7264".to_string()
                            ))),
                            Term::Not(Atom::Symbol(Symbol::Constant(
                                "CPU_SUBTYPE_SH7269".to_string()
                            ))),
                        ))))
                    }),
                )
            }
        ))
    )
}

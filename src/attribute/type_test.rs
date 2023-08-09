use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{parse_type, ConfigType, Type},
        AndExpression, Atom, Expression, ExpressionToken, FunctionCall, Parameter, Term,
    },
    symbol::Symbol,
    Attribute,
};

#[test]
fn test_parse_type() {
    let input = " string";
    assert_parsing_eq!(
        parse_type,
        input,
        Ok((
            "",
            Attribute::Type(ConfigType {
                r#type: Type::String(None),
                r#if: None
            },)
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
            Attribute::Type(ConfigType {
                r#type: Type::Bool(Some("TCC8000".to_string())),
                r#if: None
            },)
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
            Attribute::Type(ConfigType {
                r#type: Type::Bool(None),
                r#if: None
            },)
        ))
    )
}

// 2.6.25/kernel/power/Kconfig
#[test]
fn test_parse_type_backslash() {
    let input = r#"bool "Enable freezer for suspend to RAM/standby" \
    if ARCH_WANTS_FREEZER_CONTROL || BROKEN"#;
    assert_parsing_eq!(
        parse_type,
        input,
        Ok((
            "",
            Attribute::Type(ConfigType {
                r#type: Type::Bool(Some(
                    "Enable freezer for suspend to RAM/standby".to_string()
                )),
                r#if: Some(Expression::Expression(vec!(
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                        "ARCH_WANTS_FREEZER_CONTROL".to_string()
                    )))),
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                        "BROKEN".to_string()
                    ))),)
                )))
            },)
        ))
    )
}

#[test]
fn test_parse_def_bool() {
    assert_parsing_eq!(
        parse_type,
        "def_bool     !PCI ",
        Ok((
            " ",
            Attribute::Type(ConfigType {
                r#type: Type::DefBool(Expression::Term(AndExpression::Term(Term::Not(
                    Atom::Symbol(Symbol::Constant("PCI".to_string()))
                )))),
                r#if: None
            })
        ))
    )
}

#[test]
fn test_parse_type_if() {
    assert_parsing_eq!(
        parse_type,
        r#"def_bool     !PCI  if NET"#,
        Ok((
            "",
            Attribute::Type(ConfigType {
                r#type: Type::DefBool(Expression::Term(AndExpression::Term(Term::Not(
                    Atom::Symbol(Symbol::Constant("PCI".to_string()))
                )))),
                r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("NET".to_string()))
                ))))
            })
        ))
    )
}

// 5.19.7/arch/x86/Kconfig.assembler
#[test]
fn test_parse_def_bool_function() {
    assert_parsing_eq!(
        parse_type,
        "def_bool $(as-instr,vpmovm2b %k1$(comma)%zmm5)",
        Ok((
            "",
            Attribute::Type(ConfigType {
                r#type: Type::DefBool(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Function(FunctionCall {
                        name: "as-instr".to_string(),
                        parameters: vec!(Parameter {
                            tokens: vec!(
                                ExpressionToken::Literal("vpmovm2b".to_string()),
                                ExpressionToken::Space,
                                ExpressionToken::Literal("%k1".to_string()),
                                ExpressionToken::Function(Box::new(FunctionCall {
                                    name: "comma".to_string(),
                                    parameters: vec!()
                                })),
                                ExpressionToken::Literal("%zmm5".to_string())
                            )
                        })
                    })
                )))),
                r#if: None,
            })
        ))
    )
}

#[test]
fn test_parse_def_tristate() {
    assert_parsing_eq!(
        parse_type,
        "def_tristate m",
        Ok((
            "",
            Attribute::Type(ConfigType {
                r#type: Type::DefTristate(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("m".to_string()))
                )))),
                r#if: None
            })
        ))
    )
}

#[test]
fn test_type_to_string() {
    assert_eq!(
        "bool \"Support of rust\"",
        Type::Bool(Some("Support of rust".to_string())).to_string()
    );
    assert_eq!("tristate", Type::Tristate(None).to_string());
    assert_eq!("hex", Type::Hex(None).to_string());
    assert_eq!("int", Type::Int(None).to_string());
    assert_eq!("string", Type::String(None).to_string());
    assert_eq!(
        "def_bool y",
        Type::DefBool(Expression::Term(AndExpression::Term(Term::Atom(
            Atom::Symbol(Symbol::Constant("y".to_string()))
        ))))
        .to_string()
    );
    assert_eq!(
        "def_tristate m",
        Type::DefTristate(Expression::Term(AndExpression::Term(Term::Atom(
            Atom::Symbol(Symbol::Constant("m".to_string()))
        ))))
        .to_string()
    );
}

#[test]
fn test_type_with_prompt() {
    assert_parsing_eq!(
        parse_type,
        r#"bool "enable it for KVM""#,
        Ok((
            "",
            Attribute::Type(ConfigType {
                r#type: Type::Bool(Some("enable it for KVM".to_string())),
                r#if: None
            })
        ))
    )
}

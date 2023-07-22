use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
        r#type::{parse_type, EntryType, Type},
    },
    symbol::Symbol,
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

// 3.0.18/arch/arm/plat-tcc/Kconfig
#[test]
fn test_parse_type_with_weird_prompt() {
    let input = "bool TCC8000";
    assert_parsing_eq!(
        parse_type,
        input,
        Ok((
            "",
            EntryType {
                r#type: Type::Bool,
                prompt: Some("TCC8000".to_string()),
                r#if: None
            },
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
            EntryType {
                r#type: Type::Bool,
                prompt: None,
                r#if: None
            },
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
            EntryType {
                r#type: Type::Bool,
                prompt: Some("Enable freezer for suspend to RAM/standby".to_string()),
                r#if: Some(Expression(OrExpression::Expression(vec!(
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                        "ARCH_WANTS_FREEZER_CONTROL".to_string()
                    )))),
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                        "BROKEN".to_string()
                    )))),
                ))))
            },
        ))
    )
}

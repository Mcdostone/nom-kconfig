use crate::{
    assert_parsing_eq,
    attribute::{parse_prompt_value, AndExpression, Atom, Expression, Prompt, Term},
    Symbol,
};

#[test]
fn test_parse_prompt() {
    let input = "\"scripts/Kconfig.include\"";
    assert_parsing_eq!(
        parse_prompt_value,
        input,
        Ok(("", "scripts/Kconfig.include".to_string()))
    )
}

#[test]
fn test_parse_prompt_1() {
    let input = "\"Support in-kernel module decompression\"";
    assert_parsing_eq!(
        parse_prompt_value,
        input,
        Ok(("", "Support in-kernel module decompression".to_string()))
    )
}

// 3.0.18/arch/arm/plat-tcc/Kconfig
#[test]
fn test_parse_prompt_no_quote() {
    let input = " TCC8000";
    assert_parsing_eq!(parse_prompt_value, input, Ok(("", "TCC8000".to_string())))
}

#[test]
fn test_prompt_to_string() {
    assert_eq!(
        Prompt {
            prompt: "Support of KVM".to_string(),
            r#if: None
        }
        .to_string(),
        r#""Support of KVM""#
    );

    assert_eq!(
        Prompt {
            prompt: "Support of KVM".to_string(),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::Constant("KVM".to_string()))
            ))))
        }
        .to_string(),
        r#""Support of KVM" if KVM"#
    )
}

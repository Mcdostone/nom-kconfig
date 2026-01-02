use crate::{
    assert_parsing_eq,
    attribute::{parse_imply, AndExpression, Atom, Expression, Imply, Term},
    symbol::Symbol,
};

#[test]
fn test_parse_imply() {
    assert_parsing_eq!(
        parse_imply,
        "imply PCI",
        Ok((
            "",
            Imply {
                symbol: Symbol::NonConstant("PCI".to_string()),
                r#if: None
            }
        ))
    )
}

#[test]
fn test_imply_to_string() {
    assert_eq!(
        Imply {
            symbol: Symbol::NonConstant("PCI".to_string()),
            r#if: None
        }
        .to_string(),
        "PCI".to_string()
    );
    assert_eq!(
        Imply {
            symbol: Symbol::NonConstant("PCI".to_string()),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::Constant("64BITS".to_string()))
            ))))
        }
        .to_string(),
        "PCI if 64BITS".to_string()
    )
}

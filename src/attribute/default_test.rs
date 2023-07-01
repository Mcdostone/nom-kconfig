use crate::{
    assert_parsing_eq,
    attribute::{
        default::{parse_default, DefaultAttribute},
        expression::{Expression, Term},
    },
    symbol::Symbol,
};

#[test]
fn test_parse_default() {
    assert_parsing_eq!(
        parse_default,
        "default 0x1",
        Ok((
            "",
            DefaultAttribute {
                expression: Expression::Term(Term::Symbol(Symbol::Constant("0x1".to_string()))),
                r#if: None
            }
        ))
    )
}

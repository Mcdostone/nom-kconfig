use crate::{
    assert_parsing_eq,
    attribute::{
        def_bool::{parse_def_bool, DefBool},
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
    },
    symbol::Symbol,
};

#[test]
fn test_parse_def_bool() {
    let input = "def_bool     !PCI ";
    assert_parsing_eq!(
        parse_def_bool,
        input,
        Ok((
            " ",
            DefBool {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Not(
                    Atom::Symbol(Symbol::Constant("PCI".to_string()))
                )))),
                r#if: None
            }
        ))
    )
}

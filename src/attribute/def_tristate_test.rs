use crate::{
    assert_parsing_eq,
    attribute::{
        def_tristate::{parse_def_tristate, DefTristate},
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
    },
    symbol::Symbol,
};

#[test]
fn test_parse_def_tristate() {
    assert_parsing_eq!(
        parse_def_tristate,
        "def_tristate m",
        Ok((
            "",
            DefTristate {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("m".to_string()))
                )))),
                r#if: None
            }
        ))
    )
}

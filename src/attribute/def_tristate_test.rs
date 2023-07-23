use crate::{
    assert_parsing_eq,
    attribute::{
        parse_def_tristate, AndExpression, Atom, DefTristate, Expression, OrExpression, Term,
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

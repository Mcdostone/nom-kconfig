use crate::{
    assert_parsing_eq,
    attribute::{
        def_tristate::{parse_def_tristate, DefTristate},
        expression::{Expression, Term},
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
                expression: Expression::Term(Term::Symbol(Symbol::Constant("m".to_string()))),
                r#if: None
            }
        ))
    )
}

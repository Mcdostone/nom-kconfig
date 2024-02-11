use crate::{
    assert_parsing_eq,
    attribute::{parse_range, AndExpression, Atom, Expression, Range, Term},
    symbol::Symbol,
};

#[test]
fn test_parse_range() {
    assert_parsing_eq!(
        parse_range,
        "range 1 5",
        Ok((
            "",
            Range {
                lower_bound: Symbol::Constant("1"),
                upper_bound: Symbol::Constant("5"),
                r#if: None
            }
        ))
    )
}

#[test]
fn test_parse_range_to_string() {
    assert_eq!(
        Range {
            lower_bound: Symbol::Constant("1"),
            upper_bound: Symbol::Constant("5"),
            r#if: None
        }
        .to_string(),
        "1 5"
    );

    assert_eq!(
        Range {
            lower_bound: Symbol::Constant("1"),
            upper_bound: Symbol::Constant("5"),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::Constant("NET"))
            ))))
        }
        .to_string(),
        "1 5 if NET"
    )
}

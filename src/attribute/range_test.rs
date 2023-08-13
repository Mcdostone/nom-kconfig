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
                lhs: Symbol::Constant("1".to_string()),
                rhs: Symbol::Constant("5".to_string()),
                r#if: None
            }
        ))
    )
}

#[test]
fn test_parse_range_to_string() {
    assert_eq!(
        Range {
            lhs: Symbol::Constant("1".to_string()),
            rhs: Symbol::Constant("5".to_string()),
            r#if: None
        }
        .to_string(),
        "1 5"
    );

    assert_eq!(
        Range {
            lhs: Symbol::Constant("1".to_string()),
            rhs: Symbol::Constant("5".to_string()),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::Constant("NET".to_string()))
            ))))
        }
        .to_string(),
        "1 5 if NET"
    )
}

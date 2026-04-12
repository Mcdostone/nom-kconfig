use crate::{
    assert_parsing_eq,
    attribute::{parse_range, AndExpression, Atom, Expression, Range, Term},
    symbol::{ConstantSymbol, Symbol},
};

#[test]
fn test_parse_range() {
    assert_parsing_eq!(
        parse_range,
        "range 1 5",
        Ok((
            "",
            Range {
                lower_bound: Symbol::Constant(ConstantSymbol::Integer(1)),
                upper_bound: Symbol::Constant(ConstantSymbol::Integer(5)),
                r#if: None
            }
        ))
    )
}

#[test]
fn test_parse_range_to_string() {
    assert_eq!(
        Range {
            lower_bound: Symbol::Constant(ConstantSymbol::Integer(1)),
            upper_bound: Symbol::Constant(ConstantSymbol::Integer(5)),
            r#if: None
        }
        .to_string(),
        "1 5"
    );

    assert_eq!(
        Range {
            lower_bound: Symbol::Constant(ConstantSymbol::Integer(1)),
            upper_bound: Symbol::Constant(ConstantSymbol::Integer(5)),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::NonConstant("NET".to_string()))
            ))))
        }
        .to_string(),
        "1 5 if NET"
    )
}

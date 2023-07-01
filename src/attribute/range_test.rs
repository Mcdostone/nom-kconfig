use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{Expression, Term},
        range::{parse_range, Range},
    },
};

#[test]
fn test_parse_range() {
    let input = "range 1 5";
    assert_parsing_eq!(
        parse_range,
        input,
        Ok((
            "",
            Range {
                lhs: Expression::Term(Term::Number(1)),
                rhs: Expression::Term(Term::Number(5)),
                r#if: None
            }
        ))
    )
}

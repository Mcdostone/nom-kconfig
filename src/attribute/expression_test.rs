use crate::{
    assert_parsing_eq,
    attribute::expression::{parse_expression, Expression, Operator, RightOperand, Term},
    symbol::Symbol,
};

#[test]
fn test_parse_expression_number() {
    assert_parsing_eq!(
        parse_expression,
        "-412",
        Ok(("", Expression::Term(Term::Number(-412))))
    )
}

#[test]
fn test_parse_term() {
    assert_parsing_eq!(
        parse_expression,
        "!KVM",
        Ok((
            "",
            Expression::Term(Term::NotSymbol(Box::new(Expression::Term(Term::Symbol(
                Symbol::Constant("KVM".to_string())
            )))))
        ))
    )
}

#[test]
fn test_parse_and_expression() {
    assert_parsing_eq!(
        parse_expression,
        "KVM && INET",
        Ok((
            "",
            Expression::MultiTermExpression(
                Term::Symbol(Symbol::Constant("KVM".to_string())),
                vec!(RightOperand::Compare(
                    Operator::And,
                    Term::Symbol(Symbol::Constant("INET".to_string()))
                ))
            )
        ))
    )
}

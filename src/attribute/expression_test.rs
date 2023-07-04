use crate::{
    assert_parsing_eq,
    attribute::expression::{
        parse_expression, AndExpression, Atom, CompareOperator, Expression, OrExpression, Term,
    },
    symbol::Symbol,
};

#[test]
fn test_parse_expression_number() {
    assert_parsing_eq!(
        parse_expression,
        "-412",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                Atom::Number(-412)
            ))))
        ))
    )
}

#[test]
fn test_parse_term() {
    assert_parsing_eq!(
        parse_expression,
        "!KVM",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Term(Term::Not(
                Atom::Symbol(Symbol::Constant("KVM".to_string()))
            ))))
        ))
    )
}
/*
#[test]
fn test_parse_and_expression() {
    assert_parsing_eq!(
        parse_expression,
        "KVM && INET",
        Ok((
            "",
            Expression::Operation(
                "&&".to_string(),
                vec!(Expression::Term(Term::Symbol(Symbol::Constant("KVM".to_string()))),
                    Expression::Term(Term::Symbol(Symbol::Constant("INET".to_string())))
                )))
        ))
}
*/

#[test]
fn test_parse_depends_on_and() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA && ALPHA_LX164",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Expression(vec!(
                Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_MIATA".to_string()))),
                Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
            ))))
        ))
    )
}

#[test]
fn test_parse_depends_on_ambigus() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164",
        Ok((
            "",
            Expression(OrExpression::Expression(vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                    "ALPHA_MIATA".to_string()
                )))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                ))
            )))
        ))
    )
}

#[test]
fn test_parse_depends_on_optimization() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164 && (HELLO = world) || ALPHA_SX164 && (HELLO = world)",
        Ok(("", Expression(OrExpression::Expression(
            vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_MIATA".to_string())))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression(OrExpression::Term(AndExpression::Term(Term::Atom(Atom::Compare(Symbol::Constant("HELLO".to_string()), CompareOperator::Equal, Symbol::Constant("world".to_string())))))))),
                ))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression(OrExpression::Term(AndExpression::Term(Term::Atom(Atom::Compare(Symbol::Constant("HELLO".to_string()), CompareOperator::Equal, Symbol::Constant("world".to_string())))))))))
                )
            )
        )))))
    )
}

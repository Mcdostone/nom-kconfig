use crate::{
    assert_parsing_eq,
    attribute::expression::{parse_expression, Expression, Term},
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
fn test_parse_depends_on_ambigus() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164",
        Ok((
            "",
            Expression::Term(Term::Symbol(Symbol::Constant(
                "PCI".to_string()
            )))
        ))
    )
}


#[test]
fn test_parse_depends_on_optimization() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164 && (HELLO = world) || ALPHA_SX164 && (HELLO = world)",
        Ok((
            "",
            Expression::Term(Term::Symbol(Symbol::Constant(
                "PCI".to_string()
            )))
        ))
    )
}

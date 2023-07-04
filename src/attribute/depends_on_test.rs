use crate::{
    assert_parsing_eq,
    attribute::{
        depends_on::parse_depends_on,
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
    },
    symbol::Symbol,
    Attribute,
};

#[test]
fn test_parse_depends_on() {
    assert_parsing_eq!(
        parse_depends_on,
        "depends on PCI",
        Ok((
            "",
            Attribute::DependsOn(Expression(OrExpression::Term(AndExpression::Term(
                Term::Atom(Atom::Symbol(Symbol::Constant("PCI".to_string())))
            ))))
        ))
    )
}

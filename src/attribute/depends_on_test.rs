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

// 5.19.7/arch/powerpc/Kconfig
#[test]
fn test_parse_depends_on_weird_tab() {
    assert_parsing_eq!(
        parse_depends_on,
        "depends 	on LIVEPATCH",
        Ok((
            "",
            Attribute::DependsOn(Expression(OrExpression::Term(AndExpression::Term(
                Term::Atom(Atom::Symbol(Symbol::Constant("LIVEPATCH".to_string())))
            ))))
        ))
    )
}

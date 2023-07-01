use crate::{
    assert_parsing_eq,
    attribute::{
        depends_on::parse_depends_on,
        expression::{Expression, Term},
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
            Attribute::DependsOn(Expression::Term(Term::Symbol(Symbol::Constant(
                "PCI".to_string()
            ))))
        ))
    )
}

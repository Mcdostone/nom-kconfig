use crate::{
    assert_parsing_eq,
    attribute::{parse_depends_on, AndExpression, Atom, Expression, OrExpression, Term},
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
            Attribute::DependsOn(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::NonConstant("PCI".to_string()))
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
            Attribute::DependsOn(OrExpression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::NonConstant("LIVEPATCH".to_string()))
            ))))
        ))
    )
}

// 2.6.27.20/drivers/pcmcia/Kconfig
#[test]
fn test_parse_depends_on_backslash() {
    assert_parsing_eq!(
        parse_depends_on,
        r"depends on (ARCH_LUBBOCK || MACH_MAINSTONE || PXA_SHARPSL \
		    || MACH_ARMCORE || ARCH_PXA_PALM)",
        Ok((
            "",
            Attribute::DependsOn(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Parenthesis(Box::new(Expression::Expression(vec!(
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant(
                        "ARCH_LUBBOCK".to_string()
                    )))),
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant(
                        "MACH_MAINSTONE".to_string()
                    )))),
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant(
                        "PXA_SHARPSL".to_string()
                    )))),
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant(
                        "MACH_ARMCORE".to_string()
                    )))),
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant(
                        "ARCH_PXA_PALM".to_string()
                    )))),
                ))))
            ))))
        ))
    )
}

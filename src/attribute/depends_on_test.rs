use crate::{
    assert_parsing_eq,
    attribute::{depends_on::DependsOn, parse_depends_on, AndExpression, Atom, Expression, Term},
    symbol::Symbol,
};
#[cfg(feature = "glob-wildcard")]
use crate::{
    attribute::{expression::CompareOperand, CompareExpression, CompareOperator},
    symbol::ConstantSymbol,
};

#[test]
fn test_parse_depends_on() {
    assert_parsing_eq!(
        parse_depends_on,
        "depends on PCI",
        Ok((
            "",
            DependsOn {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::NonConstant("PCI".to_string())
                )))),
                r#if: None,
            }
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
            DependsOn {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::NonConstant("LIVEPATCH".to_string())
                )))),
                r#if: None,
            }
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
            DependsOn {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Parenthesis(
                    Box::new(Expression::Expression(vec!(
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
                    )))
                )))),
                r#if: None,
            }
        ))
    )
}

/// https://github.com/coreboot/coreboot/blob/main/payloads/external/SeaBIOS/Kconfig#L159
#[test]
#[cfg(feature = "glob-wildcard")]
fn test_parse_depends_on_with_minus_one() {
    assert_parsing_eq!(
        parse_depends_on,
        r"depends on SEABIOS_DEBUG_LEVEL = -1",
        Ok((
            "",
            DependsOn {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
                    CompareExpression {
                        left: CompareOperand::Symbol(Symbol::NonConstant(
                            "SEABIOS_DEBUG_LEVEL".to_string()
                        )),
                        operator: CompareOperator::Equal,
                        right: CompareOperand::Symbol(Symbol::Constant(ConstantSymbol::Integer(
                            -1
                        )))
                    }
                )))),
                r#if: None,
            }
        ))
    )
}

/// https://github.com/Mcdostone/nom-kconfig/issues/166
/// config USB_CDNS_SUPPORT
///    tristate "Cadence USB Support"
///    depends on USB_SUPPORT && HAS_DMA
///    depends on USB || USB_GADGET
///    depends on USB if !USB_GADGET
///    depends on USB_GADGET if !USB

#[test]
fn test_parse_depends_on_with_if_condition() {
    assert_parsing_eq!(
        parse_depends_on,
        r"depends on USB if !USB_GADGET",
        Ok((
            "",
            DependsOn {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::NonConstant("USB".to_string())
                )))),
                r#if: Some(Expression::Term(AndExpression::Term(Term::Not(
                    Atom::Symbol(Symbol::NonConstant("USB_GADGET".to_string()))
                )))),
            }
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::{
        requires::parse_requires, AndExpression, Atom, CompareExpression, CompareOperator,
        Expression, Term,
    },
    symbol::Symbol,
};

// 2.5.55/drivers/char/Kconfig
#[test]
fn test_parse_requires() {
    let input = "requires MTK_INFRACFG=y";
    assert_parsing_eq!(
        parse_requires,
        input,
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
                CompareExpression {
                    left: Symbol::NonConstant("MTK_INFRACFG".to_string()),
                    operator: CompareOperator::Equal,
                    right: Symbol::Constant("y".to_string())
                }
            ))))
        ))
    );

    assert_parsing_eq!(
        parse_requires,
        " requires  KVM",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                Symbol::NonConstant("KVM".to_string())
            ))))
        ))
    )
}

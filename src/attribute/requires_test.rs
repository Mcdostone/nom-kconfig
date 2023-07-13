use crate::{
    assert_parsing_eq,
    attribute::{select::{parse_select, Select}, requires::{parse_requires, Requires}, expression::{Expression, OrExpression, AndExpression, Term, Atom, CompareExpression, CompareOperator}}, symbol::Symbol,
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
            Requires {
                symbol: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Compare(CompareExpression { left: Symbol::Constant("MTK_INFRACFG".to_string()), operator: CompareOperator::Equal, right: Symbol::Constant("y".to_string())})
                ))))
            }
        ))
    )
}

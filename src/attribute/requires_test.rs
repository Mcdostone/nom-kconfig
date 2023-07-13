use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{
            AndExpression, Atom, CompareExpression, CompareOperator, Expression, OrExpression, Term,
        },
        requires::{parse_requires, Requires},
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
            Requires {
                symbol: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Compare(CompareExpression {
                        left: Symbol::Constant("MTK_INFRACFG".to_string()),
                        operator: CompareOperator::Equal,
                        right: Symbol::Constant("y".to_string())
                    })
                ))))
            }
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::{AndExpression, Atom, DefaultAttribute, Expression, Term},
    entry::configdefault::{parse_configdefault, ConfigDefault},
    symbol::{ConstantSymbol, Symbol},
};

#[test]
fn test_parse_configdefault() {
    let input = "configdefault FOO
    default y if BAR";
    assert_parsing_eq!(
        parse_configdefault,
        input,
        Ok((
            "",
            ConfigDefault {
                symbol: "FOO".to_string(),
                default: DefaultAttribute {
                    expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                        Symbol::Constant(ConstantSymbol::Boolean(true))
                    )))),
                    r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Symbol(Symbol::NonConstant("BAR".to_string()))
                    ))))
                }
            }
        ))
    )
}

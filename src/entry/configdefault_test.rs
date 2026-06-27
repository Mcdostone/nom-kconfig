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
                default_attributes: vec![DefaultAttribute {
                    expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                        Symbol::Constant(ConstantSymbol::Boolean(true))
                    )))),
                    r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Symbol(Symbol::NonConstant("BAR".to_string()))
                    ))))
                }]
            }
        ))
    )
}

#[test]
/// we can have multiple default attributes for a given `configdefault`` entry.
/// https://github.com/zephyrproject-rtos/zephyr/blob/main/drivers/bluetooth/hci/Kconfig.bflb#L121-L124
fn test_parse_configdefault_multiple_default_attributes() {
    let input = "configdefault BT_BUF_ACL_TX_COUNT
	default 16 if BFLB_BL70X_BLE_M16S1
	default 10 if BFLB_BL70X_BLE_M0S1T10";
    assert_parsing_eq!(
        parse_configdefault,
        input,
        Ok((
            "",
            ConfigDefault {
                symbol: "BT_BUF_ACL_TX_COUNT".to_string(),
                default_attributes: vec![
                    DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::Constant(ConstantSymbol::Integer(16)))
                        ))),
                        r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::NonConstant("BFLB_BL70X_BLE_M16S1".to_string()))
                        )))),
                    },
                    DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::Constant(ConstantSymbol::Integer(10)))
                        ))),
                        r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::NonConstant("BFLB_BL70X_BLE_M0S1T10".to_string()))
                        )))),
                    }
                ]
            }
        ))
    )
}

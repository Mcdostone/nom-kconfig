use crate::{
    assert_parsing_eq,
    attribute::{parse_select, AndExpression, Atom, Expression, Select, Term},
    Symbol,
};

#[test]
fn test_parse_select() {
    assert_parsing_eq!(
        parse_select,
        "select MTK_INFRACFG",
        Ok((
            "",
            Select {
                r#if: None,
                symbol: "MTK_INFRACFG".to_string()
            }
        ))
    )
}

#[test]
fn test_select_to_string() {
    assert_eq!(
        Select {
            r#if: None,
            symbol: "MTK_INFRACFG".to_string()
        }
        .to_string(),
        "MTK_INFRACFG"
    );

    assert_eq!(
        Select {
            r#if: Some(Expression::Term(AndExpression::Term(Term::Not(
                Atom::Symbol(Symbol::Constant("KVM".to_string()))
            )))),
            symbol: "MTK_INFRACFG".to_string()
        }
        .to_string(),
        "MTK_INFRACFG if !KVM"
    )
}

// v2.6.1/drivers/net/wireless/Kconfig
#[test]
fn test_parse_enable() {
    assert_parsing_eq!(
        parse_select,
        "enable MTK_INFRACFG",
        Ok((
            "",
            Select {
                r#if: None,
                symbol: "MTK_INFRACFG".to_string()
            }
        ))
    )
}

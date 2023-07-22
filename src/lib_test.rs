use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
        r#type::{EntryType, Type},
    },
    entry::{config::Config, r#if::If},
    kconfig::parse_kconfig,
    symbol::Symbol,
    Attribute, Entry, Kconfig,
};

#[macro_export]
macro_rules! assert_parsing_source_eq {
    ($fn:ident, $input:expr, $silent_fail: expr,  $expected:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra(
            $input,
            KconfigFile {
                fail_on_missing_source: $silent_fail,
                ..Default::default()
            },
        ))
        .map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}

// 2.6.25/drivers/ide/Kconfig
#[test]
fn test_parse_typedddb() {
    let input = r#"config BLK_DEV_IDEDMA_SFF
	bool

if PCI

endif"#;
    assert_parsing_eq!(
        parse_kconfig,
        input,
        Ok((
            "",
            Kconfig {
                file: "".to_string(),
                entries: vec!(
                    Entry::Config(Config {
                        symbol: "BLK_DEV_IDEDMA_SFF".to_string(),
                        attributes: vec!(Attribute::Type(EntryType {
                            r#type: Type::Bool,
                            prompt: None,
                            r#if: None
                        }))
                    }),
                    Entry::If(If {
                        condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::Constant("PCI".to_string()))
                        )))),
                        entries: vec!()
                    })
                )
            },
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::expression::{Expression, Term},
    entry::{
        r#if::{parse_if, If},
        source::Source,
    },
    symbol::Symbol,
    Entry,
};

#[test]
fn test_parse_if_entry() {
    let input = "if NET_VENDOR_AMD source \"$(VAR)/Kconfig\" endif";
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression::Term(Term::Symbol(Symbol::Constant(
                    "NET_VENDOR_AMD".to_string()
                ))),
                entries: vec!(Entry::Source(Source {
                    file: "$(VAR)/Kconfig".to_string(),
                    ..Default::default()
                }))
            }
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
        r#type::{EntryType, Type},
        select::Select,
    },
    entry::{
        config::Config,
        r#if::{parse_if, If},
        source::Source,
    },
    symbol::Symbol,
    Attribute, Entry,
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
                condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("NET_VENDOR_AMD".to_string()))
                )))),
                entries: vec!(Entry::Source(Source {
                    file: "$(VAR)/Kconfig".to_string(),
                    ..Default::default()
                }))
            }
        ))
    )
}

#[test]
fn test_parse_if_entry_with_config() {
    let input = "if VIRTUALIZATION

    config KVM
       bool
       select KVM_MMIO
   
   endif";
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("VIRTUALIZATION".to_string()))
                )))),
                entries: vec!(Entry::Config(Config {
                    symbol: "KVM".to_string(),
                    attributes: vec!(
                        Attribute::Type(EntryType {
                            r#type: Type::Bool,
                            prompt: None,
                            r#if: None
                        }),
                        Attribute::Select(Select {
                            symbol: "KVM_MMIO".to_string(),
                            r#if: None
                        })
                    )
                }))
            }
        ))
    )
}

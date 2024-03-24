use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        select::Select,
        AndExpression, Atom, Expression, Term,
    },
    entry::{parse_if, Config, If, Source},
    symbol::Symbol,
    Attribute, Entry, Kconfig,
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::Constant("NET_VENDOR_AMD")
                )))),
                entries: vec!(Entry::Source(Source {
                    content: Box::new("".into()),
                    kconfig: Kconfig {
                        file: "$(VAR)/Kconfig".to_string(),
                        ..Default::default()
                    }
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::Constant("VIRTUALIZATION")
                )))),
                entries: vec!(Entry::Config(Config {
                    symbol: "KVM",
                    attributes: vec!(
                        Attribute::Type(ConfigType {
                            r#type: Type::Bool(None),
                            r#if: None
                        }),
                        Attribute::Select(Select {
                            symbol: "KVM_MMIO",
                            r#if: None
                        })
                    )
                }))
            }
        ))
    )
}

use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        select::Select,
        AndExpression, Atom, Expression, Term,
    },
    entry::{parse_if, Config, If, Source},
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::NonConstant("NET_VENDOR_AMD".to_string())
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::NonConstant("VIRTUALIZATION".to_string())
                )))),
                entries: vec!(Entry::Config(Config {
                    symbol: "KVM".to_string(),
                    attributes: vec!(
                        Attribute::Type(ConfigType {
                            r#type: Type::Bool(None),
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

use crate::assert_parsing_eq;
use crate::attribute::parse_attributes;
use crate::attribute::{
    default::DefaultAttribute, imply::Imply, parse_attribute, prompt::Prompt, range::Range,
    select::Select, visible::Visible, AndExpression, Atom, Attribute, CompareExpression,
    CompareOperator, Enable, Expression, OptionValues, Requires, Term,
};
use crate::symbol::Symbol;

#[test]
fn test_parse_attribute() {
    assert_parsing_eq!(
        parse_attribute,
        "    default  m",
        Ok((
            "",
            Attribute::Default(DefaultAttribute {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::Constant("m".to_string())
                )))),
                r#if: None
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "depends on KVM",
        Ok((
            "",
            Attribute::DependsOn(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::Constant("KVM".to_string()))
            ))))
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "enable MTK_INFRACFG",
        Ok((
            "",
            Attribute::Enable(Enable {
                symbol: "MTK_INFRACFG".to_string()
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "help\n please",
        Ok(("", Attribute::Help("please".to_string())))
    );
    assert_parsing_eq!(
        parse_attribute,
        "imply KVM",
        Ok((
            "",
            Attribute::Imply(Imply {
                symbol: Symbol::Constant("KVM".to_string()),
                r#if: None
            })
        ))
    );
    assert_parsing_eq!(parse_attribute, "    modules", Ok(("", Attribute::Modules)));
    assert_parsing_eq!(
        parse_attribute,
        "    option      defconfig_list",
        Ok(("", Attribute::Option(OptionValues::DefconfigList)))
    );
    assert_parsing_eq!(
        parse_attribute,
        "prompt \"hello world\"",
        Ok((
            "",
            Attribute::Prompt(Prompt {
                prompt: "hello world".to_string(),
                r#if: None
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "    range 0 512",
        Ok((
            "",
            Attribute::Range(Range {
                lhs: Symbol::Constant("0".to_string()),
                rhs: Symbol::Constant("512".to_string()),
                r#if: None
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "    requires   MTK_INFRACFG=y",
        Ok((
            "",
            Attribute::Requires(Requires {
                symbol: Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
                    CompareExpression {
                        left: Symbol::Constant("MTK_INFRACFG".to_string()),
                        operator: CompareOperator::Equal,
                        right: Symbol::Constant("y".to_string())
                    }
                ))))
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "select KVM",
        Ok((
            "",
            Attribute::Select(Select {
                symbol: "KVM".to_string(),
                r#if: None
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "    visible",
        Ok(("", Attribute::Visible(Visible { r#if: None })))
    );
}

#[test]
fn test_parse_attributes() {
    let input = r#"
        select KVM 
        modules
        default 5"#;
    assert_parsing_eq!(
        parse_attributes,
        input,
        Ok((
            "",
            vec!(
                Attribute::Select(Select {
                    symbol: "KVM".to_string(),
                    r#if: None
                }),
                Attribute::Modules,
                Attribute::Default(DefaultAttribute {
                    expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Number(5)))),
                    r#if: None
                }),
            )
        ))
    );
}

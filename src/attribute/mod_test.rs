use crate::attribute::parse_attributes;
use crate::attribute::r#type::{ConfigType, Type};
use crate::attribute::{
    default::DefaultAttribute, imply::Imply, parse_attribute, prompt::Prompt, range::Range,
    select::Select, AndExpression, Atom, Attribute, CompareExpression, CompareOperator, Expression,
    OptionValues, Term,
};

use crate::assert_parsing_eq;
use crate::symbol::Symbol;

#[test]
fn test_parse_attribute() {
    assert_parsing_eq!(
        parse_attribute,
        "    transitional",
        Ok(("", Attribute::Transitional))
    );

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
            Attribute::Select(Select {
                symbol: "MTK_INFRACFG".to_string(),
                r#if: None
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
                lower_bound: Symbol::Constant("0".to_string()),
                upper_bound: Symbol::Constant("512".to_string()),
                r#if: None
            })
        ))
    );
    assert_parsing_eq!(
        parse_attribute,
        "    requires   MTK_INFRACFG=y",
        Ok((
            "",
            Attribute::Requires(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Compare(CompareExpression {
                    left: Symbol::Constant("MTK_INFRACFG".to_string()),
                    operator: CompareOperator::Equal,
                    right: Symbol::Constant("y".to_string())
                })
            ))))
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
        Ok(("", Attribute::Visible(None)))
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

#[test]
fn test_attributes_to_string() {
    let expression = Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
        Symbol::Constant("KVM".to_string()),
    ))));
    assert_eq!(
        Attribute::Help("help please".to_string()).to_string(),
        "help\n  help please".to_string()
    );
    assert_eq!(
        Attribute::Prompt(Prompt {
            prompt: "a prompt".to_string(),
            r#if: None
        })
        .to_string(),
        r#"prompt "a prompt""#.to_string()
    );
    assert_eq!(Attribute::Modules.to_string(), "modules".to_string());
    assert_eq!(Attribute::Optional.to_string(), "optional".to_string());
    assert_eq!(Attribute::Visible(None).to_string(), "visible".to_string());
    assert_eq!(
        Attribute::Visible(Some(expression.clone())).to_string(),
        "visible if KVM".to_string()
    );
    assert_eq!(
        Attribute::Select(Select {
            symbol: "NET".to_string(),
            r#if: None
        })
        .to_string(),
        "select NET".to_string()
    );
    assert_eq!(
        Attribute::DependsOn(expression.clone()).to_string(),
        "depends on KVM".to_string()
    );
    assert_eq!(
        Attribute::Transitional.to_string(),
        "transitional".to_string()
    );
    assert_eq!(
        Attribute::Range(Range {
            lower_bound: Symbol::Constant("0".to_string()),
            upper_bound: Symbol::Constant("15".to_string()),
            r#if: None
        })
        .to_string(),
        "range 0 15".to_string()
    );
    assert_eq!(
        Attribute::Default(DefaultAttribute {
            expression: expression.clone(),
            r#if: None
        })
        .to_string(),
        "default KVM".to_string()
    );
    assert_eq!(
        Attribute::Imply(Imply {
            symbol: Symbol::Constant("DEBUGGER".to_string()),
            r#if: None
        })
        .to_string(),
        "imply DEBUGGER".to_string()
    );
    assert_eq!(
        Attribute::Requires(expression).to_string(),
        "requires KVM".to_string()
    );
    assert_eq!(
        Attribute::Type(ConfigType {
            r#type: Type::Bool(None),
            r#if: None
        })
        .to_string(),
        "bool".to_string()
    );
    assert_eq!(
        Attribute::Option(OptionValues::DefconfigList).to_string(),
        "option defconfig_list".to_string()
    );
}

use test_case::test_case;

use crate::assert_parsing_eq;
use crate::attribute::{
    default::DefaultAttribute, imply::Imply, parse_attribute, prompt::Prompt, range::Range,
    select::Select, visible::Visible, AndExpression, Atom, Attribute, CompareExpression,
    CompareOperator, Enable, Expression, OptionValues, OrExpression, Requires, Term,
};
use crate::symbol::Symbol;

#[test_case( "    default  m", Attribute::Default(DefaultAttribute {expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("m".to_string())))))), r#if: None}); "Default")]
#[test_case( "depends on KVM",  Attribute::DependsOn(Expression(OrExpression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("KVM".to_string()))))))); "DependsOn")]
#[test_case("enable MTK_INFRACFG",  Attribute::Enable( Enable {
    symbol: "MTK_INFRACFG".to_string()
}); "Enable")]
#[test_case( "help\n please",  Attribute::Help("please".to_string()); "Help")]
#[test_case( "imply KVM", Attribute::Imply(Imply {symbol: Symbol::Constant("KVM".to_string()), r#if: None }); "Imply")]
#[test_case("    modules", Attribute::Modules; "Modules")]
#[test_case("    option      defconfig_list",Attribute::Option(OptionValues::DefconfigList); "Option")]
#[test_case( "prompt \"hello world\"",  Attribute::Prompt(Prompt {prompt: "hello world".to_string(), r#if: None }); "prompt")]
#[test_case( "    range 0 512", Attribute::Range(Range {
    lhs: Symbol::Constant("0".to_string()),
    rhs: Symbol::Constant("512".to_string()), 
    r#if: None
}); "Range")]
#[test_case("    requires   MTK_INFRACFG=y", Attribute::Requires(Requires {
    symbol: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
        Atom::Compare(CompareExpression {
            left: Symbol::Constant("MTK_INFRACFG".to_string()),
            operator: CompareOperator::Equal,
            right: Symbol::Constant("y".to_string())
        })
    ))))
}); "Requires")]
#[test_case( "select KVM", Attribute::Select(Select {symbol: "KVM".to_string(), r#if: None }); "Select")]
#[test_case( "    visible", Attribute::Visible(Visible {r#if: None}); "Visible")]
fn parse_attribute_test(input: &str, expected: Attribute) {
    assert_parsing_eq!(parse_attribute, input, Ok(("", expected)))
}

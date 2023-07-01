
use test_case::test_case;

use crate::parsing::prompt::Prompt;
use crate::parsing::imply::Imply;
use crate::parsing::def_tristate::DefTristate;
use crate::parsing::default::DefaultAttribute;
use crate::parsing::select::Select;
use crate::parsing::visible::Visible;
use crate::parsing::r#type::EntryType;
use crate::parsing::r#type::Type;
use crate::parsing::range::Range;
use crate::parsing::{
    attribute::{Attribute, parse_attribute}, def_bool::DefBool, expression::{Expression, Term}, symbol::Symbol, 
};


#[test_case( "def_tristate m",  Attribute::DefTristate(DefTristate { expression: Expression::Term(Term::Symbol(Symbol::Constant("m".to_string()))), r#if: None }))]
#[test_case( "def_bool y",  Attribute::DefBool(DefBool { expression: Expression::Term(Term::Symbol(Symbol::Constant("y".to_string()))), r#if: None }))]
#[test_case( "prompt \"hello world\"",  Attribute::Prompt(Prompt {prompt: "hello world".to_string(), r#if: None }))]
#[test_case( "help\n please",  Attribute::Help("please".to_string()))]
#[test_case( "depends on KVM",  Attribute::DependsOn(Expression::Term(Term::Symbol(Symbol::Constant("KVM".to_string())))))]
#[test_case( "select KVM", Attribute::Select(Select {symbol: Symbol::Constant("KVM".to_string()), r#if: None }))]
#[test_case( "imply KVM", Attribute::Imply(Imply {symbol: Symbol::Constant("KVM".to_string()), r#if: None }))]
#[test_case( "    default  m", Attribute::Default(DefaultAttribute {expression: Expression::Term(Term::Symbol(Symbol::Constant("m".to_string()))), r#if: None}) )]
#[test_case( "    modules", Attribute::Modules)]
#[test_case( "    visible", Attribute::Visible(Visible {r#if: None}))]
#[test_case( "    range 0 512", Attribute::Range(Range {
    lhs: Expression::Term(Term::Symbol(Symbol::Constant("0".to_string()))),
    rhs: Expression::Term(Term::Symbol(Symbol::Constant("512".to_string()))), 
    r#if: None
}))]
#[test_case( "   string", Attribute::Type(EntryType {
    r#type: Type::String,
    prompt: None,
    r#if: None
}))]
fn parse_attribute_test(input: &str, expected: Attribute) {
    assert_eq!(
        parse_attribute(input),
        Ok(("",expected))
    )
}
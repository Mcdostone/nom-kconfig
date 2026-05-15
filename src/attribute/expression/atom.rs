use crate::attribute::expression::parse_compare;
use crate::attribute::r#macro::{parse_macro, Macro};
use crate::attribute::{parse_expression, CompareExpression, Expression};
use crate::symbol::parse_symbol;
use crate::util::wsi;
use crate::{KconfigInput, Symbol};
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, sequence::delimited, IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Atom {
    Symbol(Symbol),
    Compare(CompareExpression),
    Macro(Macro),
    Parenthesis(Box<Expression>),
}

#[cfg(feature = "display")]
impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Compare(c) => write!(f, "{}", c),
            Atom::Macro(m) => write!(f, "{}", m),
            Atom::Parenthesis(d) => write!(f, "({})", d),
        }
    }
}

pub fn parse_atom(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    alt((
        wsi(parse_compare),
        map(parse_macro, Atom::Macro),
        map(
            delimited(wsi(tag("(")), parse_expression, wsi(tag(")"))),
            |expr: Expression| Atom::Parenthesis(Box::new(expr)),
        ),
        map(parse_symbol, Atom::Symbol), // needed to parse negative numbers, see test_parse_expression_number() in expression_test.rs
    ))
    .parse(input)
}

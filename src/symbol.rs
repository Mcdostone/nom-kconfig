use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::char,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::delimited,
    IResult,
};
use serde::Serialize;

use crate::KconfigInput;

use super::util::ws;

/// There are two types of symbols: constant and non-constant symbols. Non-constant symbols are the most
/// common ones and are defined with the 'config' statement. Non-constant symbols consist entirely of al-
/// phanumeric characters or underscores. Constant symbols are only part of expressions. Constant symbols
/// are always surrounded by single or double quotes. Within the quote, any other character is allowed and
/// the quotes can be escaped using ''.
#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Symbol {
    Constant(String),
    NonConstant(String),
}

pub fn parse_symbol(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    alt((
        map(parse_constant_symbol, |c: &str| {
            Symbol::Constant(c.to_string())
        }),
        map(
            delimited(ws(char('"')), take_until("\""), char('"')),
            |c: KconfigInput| Symbol::NonConstant(format!("\"{}\"", c)),
        ),
        map(
            delimited(ws(char('\'')), take_until("'"), char('\'')),
            |c: KconfigInput| Symbol::NonConstant(format!("'{}'", c)),
        ),
    ))(input)
}

pub fn parse_constant_symbol(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._"))))))),
        |c: KconfigInput| c.trim(),
    )(input)
}

#[cfg(feature = "display")]
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Constant(c) => write!(f, "{}", c),
            Symbol::NonConstant(c) => write!(f, "\"{}\"", c),
        }
    }
}

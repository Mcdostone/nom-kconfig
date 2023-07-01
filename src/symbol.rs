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

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Symbol {
    Constant(String),
    NonConstant(String),
}

impl Default for Symbol {
    fn default() -> Self {
        Self::Constant("".to_string())
    }
}

pub fn parse_symbol(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    alt((
        map(
            recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_"))))))),
            |c: KconfigInput| Symbol::Constant(c.trim().to_string()),
        ),
        map(
            delimited(ws(char('"')), take_until("\""), char('"')),
            |c: KconfigInput| Symbol::NonConstant(c.to_string()),
        ),
        map(
            delimited(ws(char('\'')), take_until("'"), char('\'')),
            |c: KconfigInput| Symbol::NonConstant(c.to_string()),
        ),
    ))(input)
}

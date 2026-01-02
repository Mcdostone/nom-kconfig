use nom::{
    branch::alt,
    bytes::{complete::take_until, tag_no_case, take_while},
    character::complete::{alphanumeric1, char, hex_digit1, one_of},
    combinator::{map, map_res, recognize},
    multi::many1,
    sequence::{delimited, pair},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::KconfigInput;

use super::util::ws;

/// There are two types of symbols: constant and non-constant symbols. Non-constant symbols are the most
/// common ones and are defined with the 'config' statement. Non-constant symbols consist entirely of al-
/// phanumeric characters or underscores. Constant symbols are only part of expressions. Constant symbols
/// are always surrounded by single or double quotes. Within the quote, any other character is allowed and
/// the quotes can be escaped using ''.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Symbol {
    Constant(String),
    NonConstant(String),
}

pub fn pouet(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._"))))))),
        |c: KconfigInput| c.trim(),
    )
    .parse(input)
}

pub fn parse_constant_symbol(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    alt((
        parse_constant_tristate,
        parse_constant_bool,
        parse_constant_hex,
        parse_constant_int,
        parse_constant_string,
    ))
    .parse(input)
}

pub fn parse_symbol(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    alt((
        parse_constant_symbol,
        map(parse_non_constant_symbol, |c: &str| {
            Symbol::NonConstant(c.to_string())
        }),
        // because of the test `test_parse_default_constant_symbol_with_numbers`
        //map(
        //    recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._"))))))),
        //    |c: KconfigInput| Symbol::Constant(c.to_string()),
        //),
    ))
    .parse(input)
}

pub fn parse_constant_int(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    map(
        alt((
            parse_int,
            delimited(ws(char('"')), parse_int, char('"')),
            delimited(ws(char('\'')), parse_int, char('\'')),
        )),
        |integer: usize| Symbol::Constant(integer.to_string()),
    )
    .parse(input)
}

pub fn parse_int(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, usize> {
    map_res(take_while(nom::AsChar::is_dec_digit), |d: KconfigInput| {
        d.fragment().parse::<usize>()
    })
    .parse(input)
}

pub fn parse_constant_bool(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    ws(alt((
        private_parse_constant_bool,
        delimited(ws(char('"')), private_parse_constant_bool, char('"')),
        delimited(ws(char('\'')), private_parse_constant_bool, char('\'')),
    )))
    .parse(input)
}

pub fn parse_constant_hex(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    ws(map(
        pair(tag_no_case("0x"), hex_digit1),
        |(prefix, v): (KconfigInput, KconfigInput)| {
            Symbol::Constant(format!("{}{}", prefix.fragment(), v.fragment()))
        },
    ))
    .parse(input)
}

pub fn parse_constant_tristate(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    ws(alt((
        parse_constant_bool,
        map(char('m'), |_| Symbol::Constant("m".to_string())),
        map(delimited(ws(char('"')), char('m'), char('"')), |_| {
            Symbol::Constant("m".to_string())
        }),
        map(delimited(ws(char('\'')), char('m'), char('\'')), |_| {
            Symbol::Constant("m".to_string())
        }),
    )))
    .parse(input)
}

pub fn parse_constant_string(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    map(
        alt((
            delimited(ws(char('"')), take_until("\""), char('"')),
            delimited(ws(char('\'')), take_until("\'"), char('\'')),
        )),
        |c: KconfigInput| Symbol::Constant(c.fragment().to_string()),
    )
    .parse(input)
}

pub fn parse_non_constant_symbol(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._"))))))),
        |c: KconfigInput| c.trim(),
    )
    .parse(input)
}

fn private_parse_constant_bool(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    alt((
        map(char('y'), |_| Symbol::Constant("y".to_string())),
        map(char('n'), |_| Symbol::Constant("n".to_string())),
    ))
    .parse(input)
}

use std::fmt::Display;

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Constant(c) => write!(f, "{}", c),
            Symbol::NonConstant(c) => write!(f, "\"{}\"", c),
        }
    }
}

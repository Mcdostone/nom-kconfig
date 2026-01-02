use super::util::ws;
use crate::number::parse_number;
use crate::KconfigInput;
use nom::bytes::tag;
use nom::combinator::complete;
use nom::{
    branch::alt,
    bytes::{tag_no_case, take_until},
    character::complete::{alphanumeric1, char, hex_digit1, one_of},
    combinator::{all_consuming, map, recognize},
    multi::many1,
    sequence::{delimited, pair},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use std::fmt::Display;

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

pub fn parse_constant_symbol(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    alt((
        all_consuming(parse_constant_tristate),
        all_consuming(parse_constant_bool),
        all_consuming(parse_constant_hex),
        all_consuming(parse_constant_int),
        all_consuming(parse_constant_string),
    ))
    .parse(input)
}

fn parse_env(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    all_consuming(complete(map(
        delimited(tag("\"$("), take_until(")\""), tag(")\"")),
        |e: KconfigInput| Symbol::NonConstant(format!("$({})", e.fragment())),
    )))
    .parse(input)
}

pub fn parse_symbol(input: KconfigInput) -> IResult<KconfigInput, Symbol> {
    let first_word = map(
        alt((
            complete(recognize(delimited(
                tag("\"$("),
                many1(alt((alphanumeric1, recognize(one_of("._"))))),
                tag(")\""),
            ))),
            recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._'\""))))))),
        )),
        |c: KconfigInput| c,
    )
    .parse(input)?;
    let ok = alt((
        parse_env,
        parse_constant_symbol,
        map(parse_non_constant_symbol, |c: &str| {
            Symbol::NonConstant(c.to_string())
        }),
        all_consuming(parse_constant_string),
    ))
    .parse(first_word.1)?;
    Ok((first_word.0, ok.1))
}

pub fn parse_constant_int(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Symbol> {
    map(
        alt((
            parse_number,
            delimited(ws(char('"')), parse_number, char('"')),
            delimited(ws(char('\'')), parse_number, char('\'')),
        )),
        |integer: i64| Symbol::Constant(integer.to_string()),
    )
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
        pair(complete(tag_no_case("0x")), hex_digit1),
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
        complete(alt((
            delimited(ws(char('"')), take_until("\""), char('"')),
            delimited(ws(char('\'')), take_until("\'"), char('\'')),
        ))),
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

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Constant(c) => write!(f, "{}", c),
            Symbol::NonConstant(c) => write!(f, "{}", c),
        }
    }
}

#[test]
fn test_parse_constant_symbol() {
    use crate::assert_parsing_eq;

    assert_parsing_eq!(
        parse_constant_symbol,
        "\"64BITS\"",
        Ok(("", Symbol::Constant("64BITS".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_symbol,
        "\"true\"",
        Ok(("", Symbol::Constant("true".to_string())))
    );
    assert_parsing_eq!(
        parse_constant_symbol,
        "\'false\'",
        Ok(("", Symbol::Constant("false".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_symbol,
        "'64BITS'",
        Ok(("", Symbol::Constant("64BITS".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_symbol,
        "'64'",
        Ok(("", Symbol::Constant("64".to_string())))
    )
}

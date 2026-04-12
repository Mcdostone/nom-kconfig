use super::util::ws;
use crate::number::parse_number;
use crate::string::{parse_first_word, parse_string};
use crate::tristate::Tristate;
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
#[cfg(feature = "display")]
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
    Constant(ConstantSymbol),
    NonConstant(String),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum ConstantSymbol {
    Integer(i64),
    Hex(String),
    Boolean(bool),
    String(String),
    Tristate(Tristate),
}

pub fn parse_constant_symbol(input: KconfigInput) -> IResult<KconfigInput, ConstantSymbol> {
    alt((
        all_consuming(parse_constant_bool),
        all_consuming(parse_constant_tristate),
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
    let first_word = parse_first_word.parse(input)?;
    let ok = alt((
        parse_env,
        map(parse_constant_symbol, Symbol::Constant),
        map(parse_non_constant_symbol, |c: &str| {
            Symbol::NonConstant(c.to_string())
        }),
        map(all_consuming(parse_constant_string), |c| {
            Symbol::Constant(c)
        }),
    ))
    .parse(first_word.1)?;
    Ok((first_word.0, ok.1))
}

pub fn parse_constant_int(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, ConstantSymbol> {
    map(parse_number, |integer: i64| {
        ConstantSymbol::Integer(integer)
    })
    .parse(input)
}

pub fn parse_constant_bool(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, ConstantSymbol> {
    ws(private_parse_constant_bool).parse(input)
}

pub fn parse_constant_hex(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, ConstantSymbol> {
    ws(map(parse_constant_hex_as_string, |e| {
        ConstantSymbol::Hex(e)
    }))
    .parse(input)
}

pub fn parse_constant_hex_as_string(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, String> {
    ws(map(
        pair(complete(tag_no_case("0x")), hex_digit1),
        |(prefix, v): (KconfigInput, KconfigInput)| {
            format!("{}{}", prefix.fragment(), v.fragment())
        },
    ))
    .parse(input)
}

pub fn parse_constant_tristate(
    input: KconfigInput<'_>,
) -> IResult<KconfigInput<'_>, ConstantSymbol> {
    ws(alt((
        map(char('m'), |_| ConstantSymbol::Tristate(Tristate::Module)),
        map(parse_constant_bool, |c| match c {
            ConstantSymbol::Boolean(true) => ConstantSymbol::Tristate(Tristate::Yes),
            ConstantSymbol::Boolean(false) => ConstantSymbol::Tristate(Tristate::No),
            _ => ConstantSymbol::Tristate(Tristate::No),
        }),
    )))
    .parse(input)
}

pub fn parse_constant_string(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, ConstantSymbol> {
    map(parse_string, ConstantSymbol::String).parse(input)
}

pub fn parse_non_constant_symbol(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._-"))))))),
        |c: KconfigInput| c.trim(),
    )
    .parse(input)
}

fn private_parse_constant_bool(
    input: KconfigInput<'_>,
) -> IResult<KconfigInput<'_>, ConstantSymbol> {
    alt((
        map(char('y'), |_| ConstantSymbol::Boolean(true)),
        map(char('n'), |_| ConstantSymbol::Boolean(false)),
    ))
    .parse(input)
}

#[cfg(feature = "display")]
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Constant(c) => write!(f, "{}", c),
            Symbol::NonConstant(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(feature = "display")]
impl Display for ConstantSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConstantSymbol::Boolean(b) => match b {
                true => write!(f, "y"),
                false => write!(f, "n"),
            },
            ConstantSymbol::Integer(i) => write!(f, "{}", i),
            ConstantSymbol::Hex(h) => write!(f, "{}", h),
            ConstantSymbol::String(s) => write!(f, "\"{}\"", s),
            ConstantSymbol::Tristate(t) => write!(f, "{}", t),
        }
    }
}

#[test]
fn test_parse_constant_symbol() {
    use crate::assert_parsing_eq;

    assert_parsing_eq!(
        parse_constant_symbol,
        "\"64BITS\"",
        Ok(("", ConstantSymbol::String("64BITS".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_symbol,
        "\"true\"",
        Ok(("", ConstantSymbol::String("true".to_string())))
    );
    assert_parsing_eq!(
        parse_constant_symbol,
        "\'false\'",
        Ok(("", ConstantSymbol::String("false".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_symbol,
        "'64BITS'",
        Ok(("", ConstantSymbol::String("64BITS".to_string())))
    );

    assert_parsing_eq!(
        parse_constant_symbol,
        "'64'",
        Ok(("", ConstantSymbol::String("64".to_string())))
    )
}

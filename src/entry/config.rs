use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::pair,
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    attribute::{
        r#type::{parse_bool_type, parse_tristate_type, parse_type},
        Attribute,
    },
    util::ws,
    KconfigInput,
};

/// This defines a config symbol.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Config {
    pub symbol: String,
    pub attributes: Vec<Attribute>,
}

#[macro_export]
macro_rules! generic_config_parser {
    ($t:ident, $tag:expr, $fn:expr) => {{
        use nom::branch::alt;
        use nom::multi::many0;
        use $crate::attribute::parse_attribute;

        map(
            pair(
                map(pair(ws(tag($tag)), ws(parse_config_symbol)), |(_, id)| id),
                many0(ws(alt(($fn, parse_attribute)))),
            ),
            |(symbol, attributes)| $t {
                symbol: symbol.to_string(),
                attributes,
            },
        )
    }};
}

pub fn parse_config_symbol(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_"))))))),
        |d: KconfigInput| d.fragment().to_owned(),
    )(input)
}

/*
pub fn parse_config_type(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    config_parser_with_type!(parse_bool_attribute)(input)
}
*/

macro_rules! config_parser {
    ($fn:expr) => {{
        generic_config_parser!(Config, "config", $fn)
    }};
}
pub fn parse_bool_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    config_parser!(parse_bool_type)(input)
}

pub fn parse_tristate_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    config_parser!(parse_tristate_type)(input)
}

pub fn parse_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    config_parser!(parse_type)(input)
}

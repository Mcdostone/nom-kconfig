use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::{many0, many1},
    sequence::{pair, preceded},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    attribute::{parse_attribute, r#type::parse_type, Attribute},
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
    )
    .parse(input)
}

pub fn parse_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    map(
        pair(
            preceded(ws(tag("config")), ws(parse_config_symbol)),
            many0(ws(alt((parse_type, parse_attribute)))),
        ),
        |(symbol, attributes)| Config {
            symbol: symbol.to_string(),
            attributes,
        },
    )
    .parse(input)
}

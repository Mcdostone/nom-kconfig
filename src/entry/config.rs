use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::pair,
    IResult,
};
use serde::Serialize;

use crate::{
    attribute::{parse_attributes, Attribute},
    util::ws,
    KconfigInput,
};

/// This defines a config symbol.
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Config {
    pub symbol: String,
    pub attributes: Vec<Attribute>,
}

pub fn parse_config_symbol(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_"))))))),
        |d: KconfigInput| d.fragment().to_owned(),
    )(input)
}

pub fn parse_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    map(
        pair(
            map(
                pair(ws(tag("config")), ws(parse_config_symbol)),
                |(_, id)| id,
            ),
            parse_attributes,
        ),
        |(symbol, attributes)| Config {
            symbol: symbol.to_string(),
            attributes,
        },
    )(input)
}

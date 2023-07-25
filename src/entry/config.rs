use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::{many0, many1},
    sequence::pair,
    IResult,
};
use serde::Serialize;

use crate::{
    attribute::{parse_attribute, parse_bool_attribute, parse_tristate_attribute, Attribute},
    util::ws,
    KconfigInput,
};

/// This defines a config symbol.
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Config {
    pub symbol: String,
    pub attributes: Vec<Attribute>,
}

macro_rules! config_parser_with_type {
    ($fn:expr) => {{
        map(
            pair(
                map(
                    pair(ws(tag("config")), ws(parse_config_symbol)),
                    |(_, id)| id,
                ),
                ws(many0($fn)),
            ),
            |(symbol, attributes)| Config {
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

pub fn parse_bool_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    config_parser_with_type!(parse_bool_attribute)(input)
}

pub fn parse_tristate_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    config_parser_with_type!(parse_tristate_attribute)(input)
}

pub fn parse_config(input: KconfigInput) -> IResult<KconfigInput, Config> {
    config_parser_with_type!(parse_attribute)(input)
}

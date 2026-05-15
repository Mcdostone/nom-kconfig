use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, preceded},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    attribute::{parse_default, DefaultAttribute},
    entry::config::parse_config_symbol,
    util::ws,
    KconfigInput,
};

/// This defines a config symbol.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct ConfigDefault {
    pub symbol: String,
    pub default: DefaultAttribute,
}

pub fn parse_configdefault(input: KconfigInput) -> IResult<KconfigInput, ConfigDefault> {
    map(
        pair(
            preceded(ws(tag("configdefault")), ws(parse_config_symbol)),
            parse_default,
        ),
        |(symbol, default_attribute)| ConfigDefault {
            symbol: symbol.to_string(),
            default: default_attribute,
        },
    )
    .parse(input)
}

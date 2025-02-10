use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::delimited,
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum OptionValues {
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "defconfig_list")
    )]
    DefconfigList,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "modules")
    )]
    Modules,
    AllNoConfigY,
    Env(String),
}

#[cfg(feature = "display")]
use std::fmt::Display;
#[cfg(feature = "display")]
impl Display for OptionValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OptionValues::DefconfigList => write!(f, "defconfig_list"),
            OptionValues::Modules => write!(f, "modules"),
            OptionValues::AllNoConfigY => write!(f, "allnoconfig_y"),
            OptionValues::Env(s) => write!(f, r#"env="{}""#, s),
        }
    }
}

pub fn parse_option(input: KconfigInput) -> IResult<KconfigInput, OptionValues> {
    map((ws(tag("option")), ws(parse_option_value)), |(_, i)| i).parse(input)
}

pub fn parse_option_value(input: KconfigInput) -> IResult<KconfigInput, OptionValues> {
    alt((
        value(OptionValues::DefconfigList, ws(tag("defconfig_list"))),
        value(OptionValues::Modules, ws(tag("modules"))),
        value(OptionValues::AllNoConfigY, ws(tag("allnoconfig_y"))),
        map(
            (
                ws(tag("env")),
                ws(tag("=")),
                delimited(tag("\""), parse_constant_symbol, tag("\"")),
            ),
            |(_, _, env)| OptionValues::Env(env.to_string()),
        ),
    ))
    .parse(input)
}

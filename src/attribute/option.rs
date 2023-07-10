use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::{delimited, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum OptionValues {
    #[serde(rename = "defconfig_list")]
    DefconfigList,
    #[serde(rename = "modules")]
    Modules,
    AllNoConfigY,
    Env(String),
}

pub fn parse_option(input: KconfigInput) -> IResult<KconfigInput, OptionValues> {
    map(
        tuple((ws(tag("option")), ws(parse_option_value))),
        |(_, i)| i,
    )(input)
}

pub fn parse_option_value(input: KconfigInput) -> IResult<KconfigInput, OptionValues> {
    alt((
        value(OptionValues::DefconfigList, ws(tag("defconfig_list"))),
        value(OptionValues::Modules, ws(tag("modules"))),
        value(OptionValues::AllNoConfigY, ws(tag("allnoconfig_y"))),
        map(
            tuple((
                ws(tag("env")),
                ws(tag("=")),
                delimited(tag("\""), parse_constant_symbol, tag("\"")),
            )),
            |(_, _, env)| OptionValues::Env(env.to_string()),
        ),
    ))(input)
}

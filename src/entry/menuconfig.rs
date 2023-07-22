use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::{
    attribute::{parse_attributes, Attribute},
    util::ws,
    KconfigInput,
};

use super::config::parse_config_symbol;

/// This is similar to the simple config entry, but it also gives a hint to front ends, that all suboptions should be displayed as a separate list of options. To make sure all the suboptions will really show up under the menuconfig entry and not outside of it, every item from the config options list must depend on the menuconfig symbol.
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct MenuConfig {
    pub symbol: String,
    pub attributes: Vec<Attribute>,
}

pub fn parse_menu_config(input: KconfigInput) -> IResult<KconfigInput, MenuConfig> {
    map(
        pair(
            map(
                pair(ws(tag("menuconfig")), ws(parse_config_symbol)),
                |(_, id)| id,
            ),
            parse_attributes,
        ),
        |(symbol, attributes)| MenuConfig {
            symbol: symbol.to_string(),
            attributes,
        },
    )(input)
}

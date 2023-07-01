use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::{
    attribute::{parse_attributes, Attribute},
    util::ws,
    KconfigInput,
};

use super::config::parse_config_symbol;

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

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct MenuConfig {
    pub symbol: String,
    pub attributes: Vec<Attribute>,
}

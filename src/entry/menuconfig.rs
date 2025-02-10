use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult, Parser};

use crate::{attribute::r#type::parse_type, generic_config_parser, util::ws, KconfigInput};

use super::{config::parse_config_symbol, Config};

/// This is similar to the simple config entry, but it also gives a hint to front ends, that all suboptions should be displayed as a separate list of options. To make sure all the suboptions will really show up under the menuconfig entry and not outside of it, every item from the config options list must depend on the menuconfig symbol.
pub type MenuConfig = Config;

pub fn parse_menu_config(input: KconfigInput) -> IResult<KconfigInput, MenuConfig> {
    generic_config_parser!(MenuConfig, "menuconfig", parse_type).parse(input)
}

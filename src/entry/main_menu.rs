use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{attribute::parse_prompt_value, util::ws, KconfigInput};

pub fn parse_main_menu(input: KconfigInput) -> IResult<KconfigInput, MainMenu> {
    map(
        pair(ws(tag("mainmenu")), ws(parse_prompt_value)),
        |(_, prompt)| MainMenu {
            prompt: prompt.to_string(),
        },
    )
    .parse(input)
}

/// This sets the config program's title bar if the config program chooses to use it. It should be placed at the top of the configuration, before any other statement.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct MainMenu {
    pub prompt: String,
}

use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::{Deserialize, Serialize};

use crate::{attribute::prompt::parse_prompt_option, util::ws, KconfigInput};

pub fn parse_main_menu(input: KconfigInput) -> IResult<KconfigInput, MainMenu> {
    map(
        pair(ws(tag("mainmenu")), ws(parse_prompt_option)),
        |(_, prompt)| MainMenu {
            prompt: prompt.to_string(),
        },
    )(input)
}

/// This sets the config program's title bar if the config program chooses to use it. It should be placed at the top of the configuration, before any other statement.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct MainMenu {
    pub prompt: String,
}

use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::{attribute::prompt::parse_prompt_option, util::ws, KconfigInput};

pub fn parse_main_menu(input: KconfigInput) -> IResult<KconfigInput, MainMenu> {
    map(
        pair(ws(tag("mainmenu")), ws(parse_prompt_option)),
        |(_, prompt)| MainMenu {
            prompt: prompt.to_string(),
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenu {
    pub prompt: String,
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    sequence::{pair, terminated},
    IResult,
};
use serde::Serialize;

use crate::{
    attribute::{
        depends_on::parse_depends_on,
        expression::Expression,
        prompt::parse_prompt_option,
        visible::{parse_visible, Visible},
        Attribute,
    },
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry};

fn parse_menu_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    many0(alt((
        ws(parse_depends_on),
        map(ws(parse_visible), Attribute::Visible),
    )))(input)
}

pub fn parse_menu(input: KconfigInput) -> IResult<KconfigInput, Menu> {
    let (input, prompt) = map(pair(ws(tag("menu")), ws(parse_prompt_option)), |(_, id)| id)(input)?;
    let (input, (attributes, blocks)) = terminated(
        pair(parse_menu_attributes, many0(ws(parse_entry))),
        ws(tag("endmenu")),
    )(input)?;
    let mut menu = Menu {
        prompt: prompt.to_string(),
        blocks,
        ..Default::default()
    };
    for attribute in attributes {
        match attribute {
            Attribute::Visible(a) => menu.visible = Some(a),
            Attribute::DependsOn(a) => menu.depends_on.push(a),
            _ => (),
        }
    }
    Ok((input, menu))
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Menu {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    pub depends_on: Vec<Expression>,
    pub blocks: Vec<Entry>,
}

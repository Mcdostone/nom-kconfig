use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, map},
    multi::many0,
    sequence::{pair, preceded, terminated},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    attribute::{
        parse_depends_on, parse_prompt_value, parse_visible, visible::Visible, Attribute,
        Expression,
    },
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry};

/// This defines a menu block, see ["Menu structure"](https://www.kernel.org/doc/html/latest/kbuild/kconfig-language.html#menu-structure) for more information. The only possible options are dependencies and "visible" attributes.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Menu {
    pub prompt: String,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub visible: Option<Visible>,
    pub depends_on: Vec<Expression>,
    pub entries: Vec<Entry>,
}

fn parse_menu_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    many0(alt((
        ws(parse_depends_on),
        map(ws(parse_visible), Attribute::Visible),
    )))
    .parse(input)
}

pub fn parse_menu(input: KconfigInput) -> IResult<KconfigInput, Menu> {
    let (input, mut menu) = map(
        preceded(
            ws(tag("menu")),
            pair(ws(parse_prompt_value), ws(parse_menu_attributes)),
        ),
        |(prompt, attributes)| {
            let mut menu = Menu {
                prompt: prompt.to_string(),
                ..Default::default()
            };
            for attribute in attributes {
                match attribute {
                    Attribute::Visible(a) => menu.visible = Some(a),
                    Attribute::DependsOn(a) => menu.depends_on.push(a),
                    _ => (),
                }
            }
            menu
        },
    )
    .parse(input)?;

    let (input, entries) =
        cut(terminated(many0(ws(parse_entry)), ws(tag("endmenu")))).parse(input)?;
    menu.entries = entries;
    Ok((input, menu))
}

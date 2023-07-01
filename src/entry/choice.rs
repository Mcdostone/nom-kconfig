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
    attribute::{optional::parse_optional, parse_attribute, Attribute},
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry};

fn parse_choice_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    ws(many0(alt((
        parse_attribute,
        map(ws(parse_optional), |_| Attribute::Optional),
    ))))(input)
}

pub fn parse_choice(input: KconfigInput) -> IResult<KconfigInput, Choice> {
    let (input, _) = tag("choice")(input)?;
    map(
        terminated(
            pair(parse_choice_attributes, many0(ws(parse_entry))),
            ws(tag("endchoice")),
        ),
        |(options, blocks)| Choice { options, blocks },
    )(input)
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Choice {
    pub options: Vec<Attribute>,
    pub blocks: Vec<Entry>,
}

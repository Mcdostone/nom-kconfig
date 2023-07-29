use nom::{
    bytes::complete::tag,
    combinator::{cut, map},
    multi::many0,
    sequence::{pair, terminated},
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    attribute::expression::{parse_if_expression, Expression},
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry};

/// This defines an if block. The dependency expression [expr]((crate::attribute::expression)) is appended to all enclosed menu entries.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct If {
    pub condition: Expression,
    pub entries: Vec<Entry>,
}

pub fn parse_if(input: KconfigInput) -> IResult<KconfigInput, If> {
    map(
        pair(
            ws(parse_if_expression),
            cut(terminated(many0(parse_entry), ws(tag("endif")))),
        ),
        |(condition, entries)| If { condition, entries },
    )(input)
}

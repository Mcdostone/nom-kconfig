use nom::{bytes::complete::tag, combinator::map, multi::many0, sequence::tuple, IResult};
use serde::Serialize;

use crate::{
    attribute::expression::{parse_if_expression, Expression},
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry};

pub fn parse_if(input: KconfigInput) -> IResult<KconfigInput, If> {
    map(
        tuple((
            ws(parse_if_expression),
            many0(parse_entry),
            ws(tag("endif")),
        )),
        |(condition, entries, _)| If { condition, entries },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub entries: Vec<Entry>,
}

use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_if_expression, Expression};

#[derive(Debug, Serialize, PartialEq, Clone, Default)]
pub struct Visible {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_visible(input: KconfigInput) -> IResult<KconfigInput, Visible> {
    map(
        tuple((ws(tag("visible")), opt(ws(parse_if_expression)))),
        |(_s, i)| Visible { r#if: i },
    )(input)
}

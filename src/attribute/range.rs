use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_expression, parse_if_expression_attribute, Expression};

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Range {
    pub lhs: Expression,
    pub rhs: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_range(input: KconfigInput) -> IResult<KconfigInput, Range> {
    map(
        tuple((
            ws(tag("range")),
            ws(parse_expression),
            ws(parse_expression),
            opt(parse_if_expression_attribute),
        )),
        |(_, l, r, i)| Range {
            lhs: l,
            rhs: r,
            r#if: i,
        },
    )(input)
}

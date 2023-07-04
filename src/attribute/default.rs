use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_expression, parse_if_expression_attribute, Expression};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefaultAttribute {
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_default(input: KconfigInput) -> IResult<KconfigInput, DefaultAttribute> {
    map(
        tuple((
            ws(tag("default")),
            ws(parse_expression),
            opt(parse_if_expression_attribute),
        )),
        |(_, e, i)| DefaultAttribute {
            expression: e,
            r#if: i,
        },
    )(input)
}

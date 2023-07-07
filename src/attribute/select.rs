use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

use super::expression::{parse_if_expression_attribute, Expression};

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Select {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_select(input: KconfigInput) -> IResult<KconfigInput, Select> {
    map(
        tuple((
            ws(tag("select")),
            ws(parse_constant_symbol),
            opt(parse_if_expression_attribute),
        )),
        |(_, s, i)| Select {
            symbol: s.to_string(),
            r#if: i,
        },
    )(input)
}

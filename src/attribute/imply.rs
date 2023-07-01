use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::ws,
    KconfigInput,
};

use super::expression::{parse_if_expression, Expression};

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Imply {
    pub symbol: Symbol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_imply(input: KconfigInput) -> IResult<KconfigInput, Imply> {
    map(
        tuple((
            ws(tag("imply")),
            ws(parse_symbol),
            opt(ws(parse_if_expression)),
        )),
        |(_, s, i)| Imply { symbol: s, r#if: i },
    )(input)
}

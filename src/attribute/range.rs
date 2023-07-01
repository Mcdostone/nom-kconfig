use nom::{
    branch::alt,
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

use super::expression::{parse_if_expression_attribute, parse_number, Expression};

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Range {
    pub lhs: Symbol,
    pub rhs: Symbol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_hs(input: KconfigInput) -> IResult<KconfigInput, (Symbol, Symbol)> {
    // TODO semantic controls: lhs < rhs
    alt((
        map(tuple((ws(parse_number), ws(parse_number))), |(l, r)| {
            (
                Symbol::Constant(l.to_string()),
                Symbol::Constant(r.to_string()),
            )
        }),
        tuple((ws(parse_symbol), ws(parse_symbol))),
    ))(input)
}

pub fn parse_range(input: KconfigInput) -> IResult<KconfigInput, Range> {
    map(
        tuple((
            ws(tag("range")),
            ws(parse_hs),
            opt(parse_if_expression_attribute),
        )),
        |(_, (l, r), i)| Range {
            lhs: l,
            rhs: r,
            r#if: i,
        },
    )(input)
}

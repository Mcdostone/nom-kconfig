use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

use super::expression::{parse_if_expression_attribute, Expression, parse_expression};

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Requires {
    pub symbol: Expression,
}

pub fn parse_requires(input: KconfigInput) -> IResult<KconfigInput, Requires> {
    map(
        tuple((
            ws(tag("requires")),
            ws(parse_expression),
        )),
        |(_, s)| Requires {
            symbol: s
        },
    )(input)
}

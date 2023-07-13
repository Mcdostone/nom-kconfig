use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_expression, Expression};

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Requires {
    pub symbol: Expression,
}

pub fn parse_requires(input: KconfigInput) -> IResult<KconfigInput, Requires> {
    map(
        tuple((ws(tag("requires")), ws(parse_expression))),
        |(_, s)| Requires { symbol: s },
    )(input)
}

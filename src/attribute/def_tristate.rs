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
pub struct DefTristate {
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

pub fn parse_def_tristate(input: KconfigInput) -> IResult<KconfigInput, DefTristate> {
    map(
        tuple((
            ws(tag("def_tristate")),
            ws(parse_expression),
            opt(parse_if_expression_attribute),
        )),
        |(_, e, i)| DefTristate {
            expression: e,
            r#if: i,
        },
    )(input)
}

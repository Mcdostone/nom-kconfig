use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::ws,
    KconfigInput,
};

use super::expression::{parse_if_expression_attribute, parse_number, Expression};

/// This allows to limit the range of possible input values for int and hex symbols. The user can only input a value which is larger than or equal to the first symbol and smaller than or equal to the second symbol.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Range {
    pub lhs: Symbol,
    pub rhs: Symbol,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

fn parse_hs(input: KconfigInput) -> IResult<KconfigInput, (Symbol, Symbol)> {
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

/// Parses a `range` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{parse_range, Range},
///     symbol::Symbol,
/// };
///
/// assert_parsing_eq!(
///     parse_range,
///     "range 1 5",
///     Ok((
///         "",
///         Range {
///             lhs: Symbol::Constant("1".to_string()),
///             rhs: Symbol::Constant("5".to_string()),
///             r#if: None
///         }
///     ))
/// )
/// ```
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

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, preceded, tuple},
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::ws,
    KconfigInput,
};

use super::expression::{parse_if_attribute, parse_number_as_str, Expression};

/// This attribute allows to limit the range of possible input values for int and hex symbols. The user can only input a value which is larger than or equal to the first symbol and smaller than or equal to the second symbol.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Range<'a> {
    pub lower_bound: Symbol<'a>,
    pub upper_bound: Symbol<'a>,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none", borrow)
    )]
    pub r#if: Option<Expression<'a>>,
}

#[cfg(feature = "display")]
impl Display for Range<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} {} if {}", self.lower_bound, self.upper_bound, i),
            None => write!(f, "{} {}", self.lower_bound, self.upper_bound),
        }
    }
}

fn parse_bounds(input: KconfigInput) -> IResult<KconfigInput, (Symbol, Symbol)> {
    alt((
        map(tuple((ws(parse_number_as_str), ws(parse_number_as_str))), |(l, r)| {
            (
                Symbol::Constant(l),
                Symbol::Constant(r),
            )
        }),
        tuple((ws(parse_symbol), ws(parse_symbol))),
    ))(input)
}

/// Parses the `range` attribute.
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
///             lower_bound: Symbol::Constant("1"),
///             upper_bound: Symbol::Constant("5"),
///             r#if: None
///         }
///     ))
/// )
/// ```
pub fn parse_range(input: KconfigInput) -> IResult<KconfigInput, Range> {
    map(
        preceded(ws(tag("range")), pair(ws(parse_bounds), parse_if_attribute)),
        |((l, r), i)| Range {
            lower_bound: l,
            upper_bound: r,
            r#if: i,
        },
    )(input)
}

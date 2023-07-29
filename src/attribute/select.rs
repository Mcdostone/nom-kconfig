use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

use super::expression::{parse_if_expression_attribute, Expression};

/// While normal dependencies reduce the upper limit of a symbol (see below), reverse dependencies can be used to force a lower limit of another symbol. The value of the current menu symbol is used as the minimal value [symbol](crate::symbol::Symbol) can be set to. If [symbol](crate::symbol::Symbol) is selected multiple times, the limit is set to the largest selection. Reverse dependencies can only be used with boolean or tristate symbols.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Select {
    pub symbol: String,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

/// Parses a `select` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
/// assert_parsing_eq,
/// attribute::select::{parse_select, Select},
/// };
///
/// assert_parsing_eq!(
///     parse_select,
///     "select MTK_INFRACFG",
///     Ok(("", Select {
///             r#if: None,
///             symbol: "MTK_INFRACFG".to_string()
///         }
///     ))
/// )
/// ```
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

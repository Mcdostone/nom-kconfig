use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

use super::expression::{parse_if_attribute, Expression};

/// While normal dependencies reduce the upper limit of a symbol, reverse dependencies can be used to force a lower limit of another symbol. The value of the current menu symbol is used as the minimal value [symbol](crate::Symbol) can be set to. If [symbol](crate::Symbol) is selected multiple times, the limit is set to the largest selection. Reverse dependencies can only be used with boolean or tristate symbols.
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

#[cfg(feature = "display")]
impl Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} if {}", self.symbol, i),
            None => write!(f, "{}", self.symbol),
        }
    }
}

/// Parses a `select` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
/// assert_parsing_eq,
/// attribute::{parse_select, Select}
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
        (
            ws(alt((tag("select"), tag("enable")))),
            ws(parse_constant_symbol),
            parse_if_attribute,
        ),
        |(_, s, i)| Select {
            symbol: s.to_string(),
            r#if: i,
        },
    )
    .parse(input)
}

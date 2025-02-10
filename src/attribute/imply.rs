use nom::{bytes::complete::tag, combinator::map, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::ws,
    KconfigInput,
};

use super::{expression::Expression, parse_if_attribute};

/// Imply` is similar to "select" as it enforces a lower limit on another symbol except that the "implied" symbol's value may still be set to n from a direct dependency or with a visible prompt.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Imply {
    pub symbol: Symbol,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

#[cfg(feature = "display")]
use std::fmt::Display;
#[cfg(feature = "display")]
impl Display for Imply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} if {}", self.symbol, i),
            None => write!(f, "{}", self.symbol),
        }
    }
}

/// This parses a `imply` attribute.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     Symbol,
///     attribute::{parse_imply, Imply}
/// };
///
/// assert_parsing_eq!(
///     parse_imply, "imply PCI",
///     Ok((
///         "",
///         Imply {
///             symbol: Symbol::Constant("PCI".to_string()),
///             r#if: None
///         }
///     ))
/// )
/// ```
pub fn parse_imply(input: KconfigInput) -> IResult<KconfigInput, Imply> {
    map(
        (ws(tag("imply")), ws(parse_symbol), parse_if_attribute),
        |(_, s, i)| Imply { symbol: s, r#if: i },
    )
    .parse(input)
}

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

/// This is similar to "select" as it enforces a lower limit on another symbol except that the "implied" symbol's value may still be set to n from a direct dependency or with a visible prompt.
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Imply {
    pub symbol: Symbol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

/// This parses a `imply` attribute.
///
/// # Example
/// ```rust
/// use nom_kconfig::{
///     assert_parsing_eq,
///     symbol::Symbol,
///     attribute::imply::{parse_imply, Imply}
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
        tuple((
            ws(tag("imply")),
            ws(parse_symbol),
            opt(ws(parse_if_expression)),
        )),
        |(_, s, i)| Imply { symbol: s, r#if: i },
    )(input)
}

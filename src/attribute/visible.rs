use super::expression::{parse_if_expression, Expression};
use crate::{util::ws, KconfigInput};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::{Deserialize, Serialize};

/// The `visible` attribute is only applicable to menu blocks, if the condition is false, the menu block is not displayed to the user (the symbols contained there can still be selected by other symbols, though). It is similar to a conditional "prompt" attribute for individual menu entries. Default value of "visible" is true.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Visible {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

/// Parses a `visible` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
/// assert_parsing_eq,
/// attribute::{parse_visible, Visible},
/// };
/// assert_parsing_eq!(parse_visible, "visible", Ok(("", Visible { r#if: None })))
/// ```
pub fn parse_visible(input: KconfigInput) -> IResult<KconfigInput, Visible> {
    map(
        tuple((ws(tag("visible")), opt(ws(parse_if_expression)))),
        |(_s, i)| Visible { r#if: i },
    )(input)
}

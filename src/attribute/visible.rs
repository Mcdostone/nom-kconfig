//! The `visible` attribute is only applicable to menu blocks, if the condition is false, the menu block is not displayed to the user (the symbols contained there can still be selected by other symbols, though). It is similar to a conditional "prompt" attribute for individual menu entries. Default value of "visible" is true.

use super::{parse_if_attribute, Expression};
use crate::{util::ws, KconfigInput};
use nom::{bytes::complete::tag, sequence::preceded, IResult, Parser};

pub type Visible = Option<Expression>;
/// Parses a `visible` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
/// assert_parsing_eq,
/// attribute::{parse_visible},
/// };
/// assert_parsing_eq!(parse_visible, "visible", Ok(("", None)))
/// ```
pub fn parse_visible(input: KconfigInput) -> IResult<KconfigInput, Visible> {
    preceded(ws(tag("visible")), parse_if_attribute).parse(input)
}

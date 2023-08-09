use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::char,
    character::complete::{alphanumeric1, line_ending, multispace1, not_line_ending, one_of},
    combinator::{eof, map, recognize, verify},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_if_attribute, Expression};

pub fn parse_prompt(input: KconfigInput) -> IResult<KconfigInput, Prompt> {
    map(
        tuple((ws(tag("prompt")), parse_prompt_option, parse_if_attribute)),
        |(_, p, i)| Prompt {
            prompt: p.to_string(),
            r#if: i,
        },
    )(input)
}

/// Parses a `prompt` attribute.
/// # Example
/// ```
/// use nom_kconfig::{assert_parsing_eq, attribute::parse_prompt_option};
///
/// assert_parsing_eq!(parse_prompt_option, "scripts/Kconfig.include", Ok(("", "scripts/Kconfig.include".to_string())))
/// ```
pub fn parse_prompt_option(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        alt((
            delimited(
                ws(char('"')),
                recognize(ws(many1(alt((
                    alphanumeric1,
                    multispace1,
                    tag("\\\""),
                    // TODO
                    //recognize(anychar),
                    recognize(one_of("&#*|!É{}^<>%[]()+'=,:;μ-?._$/")),
                ))))),
                char('"'),
            ),
            delimited(ws(char('\'')), take_until("'"), char('\'')),
            // TODO linux v-3.2, in file /arch/arm/plat-tcc/Kconfig
            verify(
                terminated(not_line_ending, alt((line_ending, eof))),
                |d: &KconfigInput| !d.trim().is_empty(),
            ),
        )),
        |d: KconfigInput| d.fragment().to_owned().trim().to_string(),
    )(input)
}

/// Every menu entry can have at most one prompt, which is used to display to the user. Optionally dependencies only for this prompt can be added with "if".
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Prompt {
    pub prompt: String,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

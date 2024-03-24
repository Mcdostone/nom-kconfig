use crate::Attribute;
use crate::{util::ws, KconfigInput};
use nom::sequence::pair;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::preceded,
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

use super::{parse_expression, parse_if_attribute, parse_prompt_value, Expression};

pub fn parse_type(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    map(
        pair(
            ws(alt((
                map(
                    preceded(tag("boolean"), opt(map(parse_prompt_value, |d| d.to_string()))),
                    Type::Bool,
                ),
                map(preceded(tag("bool"), opt(map(parse_prompt_value, |d| d.to_string()))), Type::Bool),
                map(preceded(tag("hex"), opt(map(parse_prompt_value, |d| d.to_string()))), Type::Hex),
                map(preceded(tag("int"), opt(map(parse_prompt_value, |d| d.to_string()))), Type::Int),
                map(
                    preceded(tag("string"), opt(map(parse_prompt_value, |d| d.to_string()))),
                    Type::String,
                ),
                map(
                    preceded(tag("tristate"), opt(map(parse_prompt_value, |d| d.to_string()))),
                    Type::Tristate,
                ),
                map(preceded(tag("def_bool"), ws(parse_expression)), |e| {
                    Type::DefBool(e)
                }),
                map(preceded(tag("def_tristate"), ws(parse_expression)), |e| {
                    Type::DefTristate(e)
                }),
            ))),
            parse_if_attribute,
        ),
        |(t, i)| Attribute::Type(ConfigType { r#type: t, r#if: i }),
    )(input)
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "lowercase")
)]
pub enum Type<'a> {
    #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(borrow))]
    DefBool(Expression<'a>),
    DefTristate(Expression<'a>),
    Bool(Option<String>),
    Tristate(Option<String>),
    String(Option<String>),
    Hex(Option<String>),
    Int(Option<String>),
}

/// Every config option must have a type. There are only two basic types: tristate and string; the other types are based on these two. The type definition optionally accepts an input prompt.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct ConfigType<'a> {
    pub r#type: Type<'a>,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none", borrow)
    )]
    pub r#if: Option<Expression<'a>>,
}

#[cfg(feature = "display")]
impl Display for ConfigType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} if {}", self.r#type, i),
            None => write!(f, "{}", self.r#type),
        }
    }
}

#[cfg(feature = "display")]
impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Bool(prompt) => fmt_type(f, "bool", prompt),
            Type::Tristate(prompt) => fmt_type(f, "tristate", prompt),
            Type::String(prompt) => fmt_type(f, "string", prompt),
            Type::Hex(prompt) => fmt_type(f, "hex", prompt),
            Type::Int(prompt) => fmt_type(f, "int", prompt),
            Type::DefBool(v) => write!(f, "def_bool {}", v),
            Type::DefTristate(v) => write!(f, "def_tristate {}", v),
        }
    }
}

#[cfg(feature = "display")]
fn fmt_type(
    f: &mut std::fmt::Formatter,
    keyword: &str,
    prompt: &Option<String>,
) -> std::fmt::Result {
    match prompt {
        Some(p) => write!(f, "{} \"{}\"", keyword, p),
        None => write!(f, "{}", keyword),
    }
}

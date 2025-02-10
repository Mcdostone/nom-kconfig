use crate::Attribute;
use crate::{util::ws, KconfigInput};
use nom::sequence::pair;
use nom::Parser;
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
                    preceded(tag("boolean"), opt(parse_prompt_value)),
                    Type::Bool,
                ),
                map(preceded(tag("bool"), opt(parse_prompt_value)), Type::Bool),
                map(preceded(tag("hex"), opt(parse_prompt_value)), Type::Hex),
                map(preceded(tag("int"), opt(parse_prompt_value)), Type::Int),
                map(
                    preceded(tag("string"), opt(parse_prompt_value)),
                    Type::String,
                ),
                map(
                    preceded(tag("tristate"), opt(parse_prompt_value)),
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
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "lowercase")
)]
pub enum Type {
    DefBool(Expression),
    DefTristate(Expression),
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
pub struct ConfigType {
    pub r#type: Type,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

#[cfg(feature = "display")]
impl Display for ConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} if {}", self.r#type, i),
            None => write!(f, "{}", self.r#type),
        }
    }
}

#[cfg(feature = "display")]
impl Display for Type {
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

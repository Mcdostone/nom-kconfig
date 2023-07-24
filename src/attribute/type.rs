use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::{parse_if_expression_attribute, parse_prompt_option, Expression};

pub fn parse_type(input: KconfigInput) -> IResult<KconfigInput, EntryType> {
    map(
        tuple((
            alt((
                map(ws(tag("boolean")), |_| Type::Bool),
                map(ws(tag("bool")), |_| Type::Bool),
                map(ws(tag("hex")), |_| Type::Hex),
                map(ws(tag("int")), |_| Type::Int),
                map(ws(tag("string")), |_| Type::String),
                map(ws(tag("tristate")), |_| Type::Tristate),
            )),
            opt(map(parse_prompt_option, |o| o.to_string())),
            opt(parse_if_expression_attribute),
        )),
        |(he, wo, e)| EntryType {
            r#type: he,
            prompt: wo,
            r#if: e,
        },
    )(input)
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Bool,
    Tristate,
    String,
    Hex,
    Int,
}

/// Every config option must have a type. There are only two basic types: tristate and string; the other types are based on these two. The type definition optionally accepts an input prompt.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct EntryType {
    pub r#type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Bool => "bool".to_string(),
            Type::Tristate => "tristate".to_string(),
            Type::String => "string".to_string(),
            Type::Hex => "hex".to_string(),
            Type::Int => "int".to_string(),
        }
    }
}

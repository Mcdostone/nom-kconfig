use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::{parse_expression, parse_if_expression_attribute, parse_prompt_option, Expression};

pub fn parse_type(input: KconfigInput) -> IResult<KconfigInput, ConfigType> {
    map(
        tuple((
            ws(alt((
                value(Type::Bool, tag("boolean")),
                value(Type::Bool, tag("bool")),
                value(Type::Hex, tag("hex")),
                value(Type::Int, tag("int")),
                value(Type::String, tag("string")),
                value(Type::Tristate, tag("tristate")),
                map(preceded(tag("def_bool"), ws(parse_expression)), |e| {
                    Type::DefBool(e)
                }),
                map(preceded(tag("def_tristate"), ws(parse_expression)), |e| {
                    Type::DefTristate(e)
                }),
            ))),
            opt(map(parse_prompt_option, |o| o.to_string())),
            opt(parse_if_expression_attribute),
        )),
        |(he, wo, e)| ConfigType {
            r#type: he,
            prompt: wo,
            r#if: e,
        },
    )(input)
}

macro_rules! parse_config_type {
    ($fn:expr) => {{
        map(
            tuple((
                ws($fn),
                opt(map(parse_prompt_option, |o| o.to_string())),
                opt(parse_if_expression_attribute),
            )),
            |(he, wo, e)| ConfigType {
                r#type: he,
                prompt: wo,
                r#if: e,
            },
        )
    }};
}

pub fn parse_bool_type(input: KconfigInput) -> IResult<KconfigInput, ConfigType> {
    parse_config_type!(alt((
        ws(value(Type::Bool, tag("boolean"))),
        ws(value(Type::Bool, tag("bool")))
    )))(input)
}

pub fn parse_tristate_type(input: KconfigInput) -> IResult<KconfigInput, ConfigType> {
    parse_config_type!(value(Type::Tristate, ws(tag("tristate"))))(input)
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    DefBool(Expression),
    DefTristate(Expression),
    Bool,
    Tristate,
    String,
    Hex,
    Int,
}

/// Every config option must have a type. There are only two basic types: tristate and string; the other types are based on these two. The type definition optionally accepts an input prompt.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ConfigType {
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
            Type::DefBool(v) => format!("{} {}", "def_bool", v.to_string()),
            Type::DefTristate(v) => format!("{} {}", "def_tristate", v.to_string()),
        }
    }
}

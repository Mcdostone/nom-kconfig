use crate::parse_config_type;
use crate::{util::ws, KconfigInput};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use super::{parse_expression, parse_if_expression_attribute, parse_prompt_option, Expression};

pub fn parse_type(input: KconfigInput) -> IResult<KconfigInput, ConfigType> {
    parse_config_type!(alt((
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
        })
    )))(input)
}

#[macro_export]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigType {
    pub r#type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

#[cfg(feature = "display")]
use std::fmt::Display;
#[cfg(feature = "display")]
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Tristate => write!(f, "tristate"),
            Type::String => write!(f, "string"),
            Type::Hex => write!(f, "hex"),
            Type::Int => write!(f, "int"),
            Type::DefBool(v) => write!(f, "def_bool {}", v),
            Type::DefTristate(v) => write!(f, "def_tristate {}", v),
        }
    }
}

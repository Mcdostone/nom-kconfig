use crate::attribute::{parse_function_call, FunctionCall};
use crate::util::ws;
use crate::KconfigInput;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, one_of};
use nom::combinator::{map, recognize};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::{IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename = "CompareOperand"))]
pub enum Macro {
    FunctionCall(FunctionCall),
    DoubleQuoted(Box<Macro>),
    Variable(String),
}

#[cfg(feature = "display")]
impl Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Macro::FunctionCall(func) => write!(f, "{}", func),
            Macro::Variable(v) => write!(f, "$({})", v),
            Macro::DoubleQuoted(d) => write!(f, "\"{})\"", d),
        }
    }
}

pub fn parse_macro(input: KconfigInput) -> IResult<KconfigInput, Macro> {
    alt((
        map(parse_function_call, Macro::FunctionCall),
        map(parse_macro_variable, Macro::Variable),
        map(delimited(tag("\""), parse_macro, tag("\"")), |e| {
            Macro::DoubleQuoted(Box::new(e))
        }),
    ))
    .parse(input)
}

pub fn parse_macro_variable(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        delimited(
            tag("$("),
            recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_-"))))))),
            tag(")"),
        ),
        |d: KconfigInput| d.fragment().to_string(),
    )
    .parse(input)
}

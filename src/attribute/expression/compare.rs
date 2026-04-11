use crate::attribute::Atom;
#[cfg(feature = "coreboot")]
use crate::attribute::{parse_function_call, FunctionCall};
#[cfg(feature = "coreboot")]
use crate::number::parse_number;
use crate::symbol::parse_symbol;
use crate::util::wsi;
use crate::{KconfigInput, Symbol};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
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
pub enum CompareOperator {
    GreaterThan,
    GreaterOrEqual,
    LowerThan,
    LowerOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename = "Compare"))]
pub struct CompareExpression {
    pub left: CompareOperand,
    pub operator: CompareOperator,
    pub right: CompareOperand,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename = "CompareOperand"))]
pub enum CompareOperand {
    Symbol(Symbol),
    #[cfg(feature = "coreboot")]
    Macro(FunctionCall),
    #[cfg(feature = "coreboot")]
    Number(i64),
}

#[cfg(feature = "display")]
impl Display for CompareOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompareOperand::Symbol(symbol) => write!(f, "{}", symbol),
            #[cfg(feature = "coreboot")]
            CompareOperand::Macro(function_call) => write!(f, "{}", function_call),
            #[cfg(feature = "coreboot")]
            CompareOperand::Number(number) => write!(f, "{}", number),
        }
    }
}

pub fn parse_compare_operand(input: KconfigInput) -> IResult<KconfigInput, CompareOperand> {
    alt((
        map(parse_symbol, CompareOperand::Symbol),
        #[cfg(feature = "coreboot")]
        map(parse_function_call, CompareOperand::Macro),
        #[cfg(feature = "coreboot")]
        map(parse_number, CompareOperand::Number),
    ))
    .parse(input)
}

#[cfg(feature = "display")]
impl Display for CompareExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

#[cfg(feature = "display")]
impl Display for CompareOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompareOperator::GreaterThan => write!(f, ">"),
            CompareOperator::GreaterOrEqual => write!(f, ">="),
            CompareOperator::LowerThan => write!(f, "<"),
            CompareOperator::LowerOrEqual => write!(f, "<="),
            CompareOperator::Equal => write!(f, "="),
            CompareOperator::NotEqual => write!(f, "!="),
        }
    }
}

pub fn parse_compare_operator(input: KconfigInput) -> IResult<KconfigInput, CompareOperator> {
    alt((
        value(CompareOperator::GreaterOrEqual, tag(">=")),
        value(CompareOperator::LowerOrEqual, tag("<=")),
        value(CompareOperator::GreaterThan, tag(">")),
        value(CompareOperator::LowerThan, tag("<")),
        value(CompareOperator::Equal, tag("=")),
        value(CompareOperator::NotEqual, tag("!=")),
    ))
    .parse(input)
}

pub fn parse_compare(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    map(
        (
            wsi(parse_compare_operand),
            wsi(parse_compare_operator),
            wsi(parse_compare_operand),
        ),
        |(l, o, r)| {
            Atom::Compare(CompareExpression {
                left: l,
                operator: o,
                right: r,
            })
        },
    )
    .parse(input)
}

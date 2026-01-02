use crate::attribute::Atom;
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
    pub left: Symbol,
    pub operator: CompareOperator,
    pub right: Symbol,
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
            wsi(parse_symbol),
            wsi(parse_compare_operator),
            wsi(parse_symbol),
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

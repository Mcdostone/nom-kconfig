pub mod atom;
pub mod compare;
#[cfg(test)]
mod mod_test;
pub mod term;

pub use atom::*;
pub use compare::*;
pub use term::*;

#[cfg(feature = "display")]
use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many0,
    sequence::{pair, preceded},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{util::wsi, KconfigInput};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Operator {
    And,
    Or,
}

// https://stackoverflow.com/questions/9509048/antlr-parser-for-and-or-logic-how-to-get-expressions-between-logic-operators
pub type Expression = OrExpression;
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum AndExpression {
    #[cfg_attr(feature = "serialize", serde(rename = "AndTerm"))]
    Term(Term),
    #[cfg_attr(feature = "serialize", serde(rename = "And"))]
    Expression(Vec<Term>),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum OrExpression {
    #[cfg_attr(feature = "serialize", serde(rename = "OrTerm"))]
    Term(AndExpression),
    #[cfg_attr(feature = "serialize", serde(rename = "Or"))]
    Expression(Vec<AndExpression>),
}

#[cfg(feature = "display")]
impl Display for AndExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Term(t) => write!(f, "{}", t),
            Self::Expression(t) => write!(
                f,
                "{}",
                t.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(" && ")
            ),
        }
    }
}

#[cfg(feature = "display")]
impl Display for OrExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Term(t) => write!(f, "{}", t),
            Self::Expression(t) => write!(
                f,
                "{}",
                t.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(" || ")
            ),
        }
    }
}

pub fn parse_or_expression(input: KconfigInput) -> IResult<KconfigInput, OrExpression> {
    map(
        (
            wsi(parse_and_expression),
            many0(preceded(wsi(tag("||")), wsi(parse_and_expression))),
        ),
        |(l, ee)| {
            if ee.is_empty() {
                OrExpression::Term(l)
            } else {
                let mut ll = vec![l];
                ll.extend(ee);
                OrExpression::Expression(ll)
            }
        },
    )
    .parse(input)
}

pub fn parse_and_expression(input: KconfigInput) -> IResult<KconfigInput, AndExpression> {
    map(
        (
            wsi(parse_term),
            many0(preceded(wsi(tag("&&")), wsi(parse_term))),
        ),
        |(l, ee)| {
            if ee.is_empty() {
                AndExpression::Term(l)
            } else {
                let mut ll = vec![l];
                ll.extend(ee);
                AndExpression::Expression(ll)
            }
        },
    )
    .parse(input)
}

pub fn parse_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    parse_or_expression(input)
}

pub fn parse_if_attribute(input: KconfigInput) -> IResult<KconfigInput, Option<Expression>> {
    opt(parse_if_expression).parse(input)
}

pub fn parse_if_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(pair(wsi(tag("if")), wsi(parse_expression)), |(_, e)| e).parse(input)
}

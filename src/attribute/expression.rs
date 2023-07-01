use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt, recognize, value},
    multi::many1,
    sequence::{delimited, pair, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::ws,
    KconfigInput,
};

use super::function::{parse_function_call, FunctionCall};

// (GFS2_FS!=n) && NET && INET && (IPV6 || IPV6=n) && CONFIGFS_FS && SYSFS && (DLM=y || DLM=GFS2_FS)

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Operator {
    GreaterThan,
    GreaterOrEqual,
    LowerThan,
    LowerOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Expression {
    Term(Term),
    MultiTermExpression(Term, Vec<RightOperand>),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum RightOperand {
    Compare(Operator, Term),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Term {
    Symbol(Symbol),
    Number(i64),
    NotSymbol(Box<Expression>),
    Function(FunctionCall),
    Parenthesis(Box<Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Term(Default::default())
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::Symbol(Default::default())
    }
}

pub fn parse_right_operand(input: KconfigInput) -> IResult<KconfigInput, RightOperand> {
    map(pair(ws(parse_operator), ws(parse_term)), |(o, t)| {
        RightOperand::Compare(o, t)
    })(input)
}

pub fn parse_operator(input: KconfigInput) -> IResult<KconfigInput, Operator> {
    alt((
        value(Operator::GreaterOrEqual, tag(">=")),
        value(Operator::LowerOrEqual, tag("<=")),
        value(Operator::GreaterThan, tag(">")),
        value(Operator::LowerThan, tag("<")),
        value(Operator::Equal, tag("=")),
        value(Operator::NotEqual, tag("!=")),
        value(Operator::And, tag("&&")),
        value(Operator::Or, tag("||")),
    ))(input)
}

pub fn parse_term(input: KconfigInput) -> IResult<KconfigInput, Term> {
    alt((
        map(pair(ws(tag("!")), parse_expression), |(_, o)| {
            Term::NotSymbol(Box::new(o))
        }),
        map(parse_function_call, Term::Function),
        map(
            delimited(ws(tag("(")), parse_expression, ws(tag(")"))),
            |expr| Term::Parenthesis(Box::new(expr)),
        ),
        map(parse_symbol, Term::Symbol),
        map(parse_number, Term::Number),
    ))(input)
}

pub fn parse_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    alt((
        map(
            pair(ws(parse_term), many1(parse_right_operand)),
            |(l, o)| Expression::MultiTermExpression(l, o),
        ),
        map(ws(parse_term), Expression::Term),
    ))(input)
}

pub fn parse_if_expression_attribute(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(
        tuple((space1, tag("if"), ws(parse_expression))),
        |(_, _, e)| e,
    )(input)
}

pub fn parse_hex_number(input: KconfigInput) -> IResult<KconfigInput, i64> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |d: KconfigInput| FromStr::from_str(d.fragment()),
    )(input)
}


pub fn parse_number(input: KconfigInput) -> IResult<KconfigInput, i64> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |d: KconfigInput| FromStr::from_str(d.fragment()),
    )(input)
}

pub fn parse_if_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(pair(ws(tag("if")), ws(parse_expression)), |(_, e)| e)(input)
}

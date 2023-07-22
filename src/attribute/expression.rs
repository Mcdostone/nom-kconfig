use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::wsi,
    KconfigInput,
};

use super::function::{parse_function_call, FunctionCall};

// (GFS2_FS!=n) && NET && INET && (IPV6 || IPV6=n) && CONFIGFS_FS && SYSFS && (DLM=y || DLM=GFS2_FS)

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum CompareOperator {
    GreaterThan,
    GreaterOrEqual,
    LowerThan,
    LowerOrEqual,
    Equal,
    NotEqual,
}

// https://stackoverflow.com/questions/9509048/antlr-parser-for-and-or-logic-how-to-get-expressions-between-logic-operators

#[derive(Debug, Serialize, PartialEq, Clone, Default)]
pub struct Expression(pub OrExpression);
#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum AndExpression {
    #[serde(rename = "AndTerm")]
    Term(Term),

    #[serde(rename = "And")]
    Expression(Vec<Term>),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum OrExpression {
    #[serde(rename = "OrTerm")]
    Term(AndExpression),
    #[serde(rename = "Or")]
    Expression(Vec<AndExpression>),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Term {
    Not(Atom),
    Atom(Atom),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename = "Compare")]
pub struct CompareExpression {
    pub left: Symbol,
    pub operator: CompareOperator,
    pub right: Symbol,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Atom {
    Symbol(Symbol),
    Number(i64),
    Compare(CompareExpression),
    Function(FunctionCall),
    Parenthesis(Box<Expression>),
    String(Box<Atom>),
}

impl Default for OrExpression {
    fn default() -> Self {
        Self::Term(Default::default())
    }
}

impl Default for AndExpression {
    fn default() -> Self {
        Self::Term(Default::default())
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::Atom(Default::default())
    }
}
impl Default for Atom {
    fn default() -> Self {
        Self::Symbol(Default::default())
    }
}

pub fn parse_or_expression(input: KconfigInput) -> IResult<KconfigInput, OrExpression> {
    map(
        tuple((
            wsi(parse_and_expression),
            many0(preceded(wsi(tag("||")), wsi(parse_and_expression))),
        )),
        |(l, ee)| {
            if ee.is_empty() {
                OrExpression::Term(l)
            } else {
                let mut ll = vec![l];
                ll.extend(ee);
                OrExpression::Expression(ll)
            }
        },
    )(input)
}

pub fn parse_and_expression(input: KconfigInput) -> IResult<KconfigInput, AndExpression> {
    map(
        tuple((
            wsi(parse_term),
            many0(preceded(wsi(tag("&&")), wsi(parse_term))),
        )),
        |(l, ee)| {
            if ee.is_empty() {
                AndExpression::Term(l)
            } else {
                let mut ll = vec![l];
                ll.extend(ee);
                AndExpression::Expression(ll)
            }
        },
    )(input)
}

pub fn parse_term(input: KconfigInput) -> IResult<KconfigInput, Term> {
    alt((
        map(preceded(wsi(tag("!")), parse_atom), Term::Not),
        map(parse_atom, Term::Atom),
    ))(input)
}

pub fn parse_atom(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    alt((
        wsi(parse_compare),
        map(parse_function_call, Atom::Function),
        map(delimited(tag("\""), parse_atom, tag("\"")), |d| {
            Atom::String(Box::new(d))
        }),
        map(
            delimited(wsi(tag("(")), parse_expression, wsi(tag(")"))),
            |expr| Atom::Parenthesis(Box::new(expr)),
        ),
        map(parse_number, Atom::Number),
        map(parse_symbol, Atom::Symbol),
    ))(input)
}

pub fn parse_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(parse_or_expression, Expression)(input)
}

pub fn parse_compare_operator(input: KconfigInput) -> IResult<KconfigInput, CompareOperator> {
    alt((
        value(CompareOperator::GreaterOrEqual, tag(">=")),
        value(CompareOperator::LowerOrEqual, tag("<=")),
        value(CompareOperator::GreaterThan, tag(">")),
        value(CompareOperator::LowerThan, tag("<")),
        value(CompareOperator::Equal, tag("=")),
        value(CompareOperator::NotEqual, tag("!=")),
    ))(input)
}

pub fn parse_compare(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    map(
        tuple((
            wsi(parse_symbol),
            wsi(parse_compare_operator),
            wsi(parse_symbol),
        )),
        |(l, o, r)| {
            Atom::Compare(CompareExpression {
                left: l,
                operator: o,
                right: r,
            })
        },
    )(input)
}

pub fn parse_if_expression_attribute(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(tuple((wsi(tag("if")), wsi(parse_expression))), |(_, e)| e)(input)
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
    map(pair(wsi(tag("if")), wsi(parse_expression)), |(_, e)| e)(input)
}

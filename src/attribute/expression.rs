#[cfg(feature = "display")]
use std::fmt::Display;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res, opt, recognize, value},
    error::{Error, ErrorKind, ParseError},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult, Input, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::wsi,
    KconfigInput,
};

use super::function::{parse_function_call, FunctionCall};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Operator {
    And,
    Or,
}

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

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Term {
    Not(Atom),
    Atom(Atom),
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

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Atom {
    Symbol(Symbol),
    Number(i64),
    Compare(CompareExpression),
    Function(FunctionCall),
    Parenthesis(Box<Expression>),
    String(String),
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

#[cfg(feature = "display")]
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Not(atom) => write!(f, "!{}", atom),
            Term::Atom(atom) => write!(f, "{}", atom),
        }
    }
}

#[cfg(feature = "display")]
impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Number(d) => write!(f, "{}", d),
            Atom::Compare(c) => write!(f, "{}", c),
            Atom::Function(func) => write!(f, "{}", func),
            Atom::Parenthesis(d) => write!(f, "({})", d),
            Atom::String(s) => write!(f, r#""{}""#, s),
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

pub fn parse_term(input: KconfigInput) -> IResult<KconfigInput, Term> {
    alt((
        map(preceded(wsi(tag("!")), parse_atom), Term::Not),
        map(parse_atom, Term::Atom),
    ))
    .parse(input)
}

pub fn parse_atom(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    alt((
        wsi(parse_compare),
        map(parse_function_call, Atom::Function),
        map(
            delimited(wsi(tag("(")), parse_expression, wsi(tag(")"))),
            |expr: Expression| Atom::Parenthesis(Box::new(expr)),
        ),
        map(parse_string, Atom::String),
        parse_number_or_symbol,
        // needed to parse negative numbers, see test_parse_expression_number() in expression_test.rs
        map(parse_number, Atom::Number),
    ))
    .parse(input)
}

pub fn parse_string(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        delimited(tag("\""), take_until_unbalanced('"'), tag("\"")),
        |d| d.fragment().to_string(),
    )
    .parse(input)
}

pub fn take_until_unbalanced(
    delimiter: char,
) -> impl Fn(KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    move |i: KconfigInput| {
        let mut index: usize = 0;
        let mut delimiter_counter = 0;

        let end_of_line = match &i.find('\n') {
            Some(e) => *e,
            None => i.len(),
        };

        while let Some(n) = &i[index..end_of_line].find(delimiter) {
            delimiter_counter += 1;
            index += n + 1;
        }

        // we split just before the last double quote
        index -= 1;
        // Last delimiter is the string delimiter
        delimiter_counter -= 1;

        match delimiter_counter % 2 == 0 {
            true => Ok(i.take_split(index)),
            false => Err(nom::Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            ))),
        }
    }
}

pub fn parse_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    parse_or_expression(input)
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

// TODO ugly
pub fn parse_number_or_symbol(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    let (input, sym) = parse_symbol(input)?;
    match sym.clone() {
        Symbol::Constant(s) => match string_to_number(s.as_str()) {
            Ok((_, i)) => Ok((input, Atom::Number(i))),
            Err(_) => Ok((input, Atom::Symbol(sym))),
        },
        Symbol::NonConstant(_) => Ok((input, Atom::Symbol(sym))),
    }
}

pub fn string_to_number(input: &str) -> IResult<&str, i64> {
    all_consuming(map_res(recognize(pair(opt(char('-')), digit1)), |d| {
        FromStr::from_str(d)
    }))
    .parse(input)
}

pub fn parse_if_attribute(input: KconfigInput) -> IResult<KconfigInput, Option<Expression>> {
    opt(parse_if_expression).parse(input)
}

pub fn parse_if_expression(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(pair(wsi(tag("if")), wsi(parse_expression)), |(_, e)| e).parse(input)
}

pub fn parse_number(input: KconfigInput) -> IResult<KconfigInput, i64> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |d: KconfigInput| FromStr::from_str(d.fragment()),
    )
    .parse(input)
}

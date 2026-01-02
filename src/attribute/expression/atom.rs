use crate::attribute::expression::parse_compare;
use crate::attribute::{
    parse_expression, parse_function_call, CompareExpression, Expression, FunctionCall,
};
use crate::number::parse_number;
use crate::symbol::parse_symbol;
use crate::util::wsi;
use crate::{KconfigInput, Symbol};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::{Error, ErrorKind, ParseError},
    sequence::{delimited, pair},
    IResult, Input, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;
use std::str::FromStr;

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

pub fn parse_atom(input: KconfigInput) -> IResult<KconfigInput, Atom> {
    alt((
        wsi(parse_compare),
        map(parse_function_call, Atom::Function),
        map(
            delimited(wsi(tag("(")), parse_expression, wsi(tag(")"))),
            |expr: Expression| Atom::Parenthesis(Box::new(expr)),
        ),
        parse_number_or_symbol,
        // needed to parse negative numbers, see test_parse_expression_number() in expression_test.rs
        map(parse_number, Atom::Number),
        map(parse_string, Atom::String),
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

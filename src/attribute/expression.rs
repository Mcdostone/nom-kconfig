use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0},
    sequence::{delimited, pair, tuple, preceded},
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
    And,
    Or,
}
impl Operator {
    fn to_string(&self) -> &str {
        match self {
            Operator::And => "&&",
            Operator::Or => "||",
        }
    }
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

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Expression(OrExpression);
//#[derive(Debug, Serialize, PartialEq, Clone)]
//pub struct OrExpression(BooleanExpression<AndExpression>);
//#[derive(Debug, Serialize, PartialEq, Clone)]
//pub struct AndExpression(BooleanExpression<Term>);
#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum AndExpression {
    Term(Term),
    Expression(BooleanExpression<Term>)
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum OrExpression {
    Term(AndExpression),
    Expression(BooleanExpression<AndExpression>)
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct BooleanExpression<T> {
    operator: Operator,
    operands: Vec<T>
}

pub fn parse_or_expression(input: KconfigInput) -> IResult<KconfigInput, OrExpression> {
    map(tuple((ws(parse_and_expression), many0(preceded(ws(tag("||")), ws(parse_and_expression))))), |(l, ee)| {
        if ee.is_empty() {
            OrExpression::Term(l)
        } else {
            let mut ll = vec!(l);
            ll.extend(ee);
            OrExpression::Expression(BooleanExpression { operator: Operator::Or, operands: ll })
    
        }
    })(input)
}

pub fn parse_and_expression(input: KconfigInput) -> IResult<KconfigInput, AndExpression> {
    map(tuple((ws(parse_term), many0(preceded(ws(tag("&&")), ws(parse_term))))), |(l, ee)|
    {
        if ee.is_empty() {
            AndExpression::Term(l)
        } else {
            let mut ll = vec!(l);
            ll.extend(ee);
            AndExpression::Expression(BooleanExpression { operator: Operator::And, operands: ll })
        }
    } )(input)
}

    
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Compare {
    pub operator: Operator,
    pub term: Term,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum LeftOperand {
    Compare(Term, Operator),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Term {
    Symbol(Symbol),
    Number(i64),
    Compare(Symbol, CompareOperator, Symbol),
    Not(Box<Expression>),
    NotSymbol(Symbol),
    Function(FunctionCall),
    Parenthesis(Box<Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Expression(Default::default())
    }
}

impl<T> Default for BooleanExpression<T> {
    fn default() -> Self {
        Self {
            operator: Operator::And,
            operands: vec!(),
        }
    }
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
        Self::Symbol(Default::default())
    }
}

pub fn parse_term(input: KconfigInput) -> IResult<KconfigInput, Term> {
    alt((
        ws(parse_compare),
        
        map(pair(ws(tag("!")), parse_expression), |(_, o)| {
            Term::Not(Box::new(o))
        }),
        map(pair(ws(tag("!")), parse_symbol), |(_, o)| {
            Term::NotSymbol(o)
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
    map(parse_or_expression, |d| Expression(d))(input)
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

pub fn parse_compare(input: KconfigInput) -> IResult<KconfigInput, Term> {
    map(tuple((ws(parse_symbol), ws(parse_compare_operator), ws(parse_symbol))), |(l, o, r)| Term::Compare(l, o, r))(input)
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

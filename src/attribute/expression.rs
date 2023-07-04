use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt, recognize, value, peek},
    multi::{many1, separated_list1},
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
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Expression {
    Term(Term),
    Operation(Operation),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Operation {
    pub operator: Operator,
    pub operands: Vec<Expression>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Compare(Operator, Term);


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

impl Operator {
    fn tt(&self) -> &str {
        return match self {
            Operator::GreaterOrEqual => ">=",
            Operator::LowerOrEqual => "<=",
            Operator::GreaterThan => ">",
            Operator::LowerThan => "<",
            Operator::Equal => "=",
            Operator::NotEqual => "!=",
            Operator::And => "&&",
            Operator::Or => "||"
        }
    }
}

enum BooleanOperator {
    And, 
    Or
}
pub struct BooleanExpression(BooleanOperator, Expression, Expression);


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
        parse_operation,
        map(ws(parse_term), Expression::Term),
    ))(input)
}

pub fn dd(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    println!("{}", input.fragment());
    map(pair(
        ws(tag("||")),
        ws(parse_expression)
    ), |(_, d)| d)(input)
}

pub fn parse_operation(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    let (input, left) = parse_term(input)?;
    let (input, operator) = peek(ws(parse_operator))(input)?;
    let parse_tt = ws(tag(operator.tt()));
    let (input, operands) = many1(map(pair(parse_tt, ws(parse_expression)), |(_, d)| d))(input)?;
    let mut r = vec!(Expression::Term(left));
    r.extend(operands);
    Ok((input, Expression::Operation(Operation { operator: operator, operands: r})))
    //let o = Operation { operator: operator, operands: r};
    //return Ok((input, optimise(o)))
}


impl Operation {
    pub fn optimise(&self) -> Self {
        println!("oo");
        let mut new_operation = Operation { operator: self.operator.clone(), operands: vec!() };
        for operator in self.operands.iter() {
            match operator {
                Expression::Term(t) => {
                    new_operation.operands.push(operator.clone())
                },
                Expression::Operation(op) => {
                    let rrop: Operation = op.optimise();
                    if rrop.operator == new_operation.operator {
                        new_operation.operands.extend(rrop.operands);
                    } else {
                        new_operation.operands.push(Expression::Operation(op.clone()));
                    }
                },
            }
        }
        return new_operation
    }

}

impl Expression {

    pub fn optimise(self) -> Self {
        if let Self::Operation(o) = self {
            return Self::Operation(o.optimise())
        }
        return self
    }

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

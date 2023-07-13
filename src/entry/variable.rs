use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{
    attribute::function::{parse_expression_token_variable_parameter, ExpressionToken},
    util::{parse_until_eol, ws},
    KconfigInput,
};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct VariableAssignment {
    pub identifier: VariableIdentifier,
    pub operator: String,
    pub right: Value,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum VariableIdentifier {
    Identifier(String),
    VariableRef(Vec<ExpressionToken>),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Value {
    Literal(String),
    ExpandedVariable(String),
}

pub fn parse_value(input: KconfigInput) -> IResult<KconfigInput, Value> {
    //alt((
    /*map(
        recognize(delimited(
            tag("$("),
            many1(alt((alphanumeric1, recognize(one_of("_"))))),
            tag(")"),
        )),
        |c: KconfigInput| Value::ExpandedVariable(c.to_string()),
    ),*/
    /*map(
        ws(recognize(many1(alt((
            alphanumeric1,
            recognize(one_of("_")),
        ))))),
        |s: KconfigInput| Value::Literal(s.to_string()),
    ),*/
    map(parse_until_eol, |d| {
        Value::Literal(d.fragment().trim().to_string())
    })(input)
}

pub fn parse_variable_identifier(input: KconfigInput) -> IResult<KconfigInput, VariableIdentifier> {
    alt((
        map(
            recognize(ws(many1(alt((alphanumeric1, recognize(one_of("-_"))))))),
            |l: KconfigInput| VariableIdentifier::Identifier(l.trim().to_string()),
        ),
        map(many1(parse_expression_token_variable_parameter), |v| {
            VariableIdentifier::VariableRef(v)
        }),
    ))(input)
}

pub fn parse_variable_assignment(input: KconfigInput) -> IResult<KconfigInput, VariableAssignment> {
    map(
        tuple((
            ws(parse_variable_identifier),
            ws(parse_assign),
            ws(parse_value),
        )),
        |(l, o, r)| VariableAssignment {
            identifier: l,
            operator: o.to_string(),
            right: r,
        },
    )(input)
}

pub fn parse_assign(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(alt((tag("="), tag(":="), tag("+="))), |d: KconfigInput| {
        d.fragment().to_owned()
    })(input)
}

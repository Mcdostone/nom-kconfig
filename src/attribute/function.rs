use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, char, one_of, space1},
    combinator::{map, opt, recognize},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

#[derive(Debug, Serialize, PartialEq, Clone, Default)]
pub struct FunctionCall {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, PartialEq, Clone, Default)]
pub struct Parameter {
    pub tokens: Vec<ExpressionToken>,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum ExpressionToken {
    Literal(String),
    Variable(String),
    DoubleQuotes(Vec<ExpressionToken>),
    SingleQuotes(String),
    Backtick(String),
    Function(Box<FunctionCall>),
    Space,
}

pub fn parse_expression_token_variable_parameter(
    input: KconfigInput,
) -> IResult<KconfigInput, ExpressionToken> {
    map(
        delimited(
            tag("$("),
            recognize(ws(many1(recognize(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ_"))))),
            tag(")"),
        ),
        |d: KconfigInput| ExpressionToken::Variable(d.fragment().to_string()),
    )(input)
}

fn parse_expression_token_parameter(input: KconfigInput) -> IResult<KconfigInput, ExpressionToken> {
    alt((
        map(tag("="), |_| ExpressionToken::Literal("=".to_string())),
        map(space1, |_| ExpressionToken::Space),
        map(tag("2>"), |_| ExpressionToken::Literal("2>".to_string())),
        map(
            delimited(tag("\""), parse_expression_parameter, tag("\"")),
            ExpressionToken::DoubleQuotes,
        ),
        map(
            delimited(tag("`"), take_until("`"), tag("`")),
            |d: KconfigInput| ExpressionToken::Backtick(d.to_string()),
        ),
        map(
            delimited(
                ws(char::<KconfigInput, _>('\'')),
                take_until("'"),
                char('\''),
            ),
            |d| ExpressionToken::SingleQuotes(d.to_string()),
        ),
        parse_literal_parameter,
        parse_expression_token_variable_parameter,
        map(parse_function_call, |f| {
            ExpressionToken::Function(Box::new(f))
        }),
    ))(input)
}

fn parse_instruction_parameter(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        tuple((
            tag("%"),
            recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_"))))))),
            delimited(tag("("), alphanumeric1, tag(")")),
        )),
        |(_a, b, c)| format!("%{}({})", b, c),
    )(input)
}

fn parse_env_variable_parameter(input: KconfigInput) -> IResult<KconfigInput, ExpressionToken> {
    map(
        ws(preceded(tag("$"), recognize(many1(alphanumeric1)))),
        |d| ExpressionToken::Literal(format!("${}", d)),
    )(input)
}

fn parse_literal_parameter(input: KconfigInput) -> IResult<KconfigInput, ExpressionToken> {
    alt((
        parse_env_variable_parameter,
        map(parse_instruction_parameter, ExpressionToken::Literal),
        map(
            recognize(ws(many1(alt((
                alphanumeric1,
                tag("\\$"),
                recognize(one_of("+(<>%&\\[]_|'.-:\n\\/")),
            ))))),
            |d: KconfigInput| ExpressionToken::Literal(d.fragment().to_string()),
        ),
    ))(input)
}

pub fn parse_expression_parameter(
    input: KconfigInput,
) -> IResult<KconfigInput, Vec<ExpressionToken>> {
    alt((many1(parse_expression_token_parameter),))(input)
}

pub fn parse_parameter(input: KconfigInput) -> IResult<KconfigInput, Parameter> {
    map(
        alt((
            //map(parse_literal_parameter , |d| Parameter::Literal(d.to_string())),
            parse_expression_parameter,
        )),
        |d| Parameter { tokens: d },
    )(input)
}

fn parse_function_name(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("=-"))))))),
        |d: KconfigInput| d.fragment().to_owned(),
    )(input)
}

pub fn parse_function_call(input: KconfigInput) -> IResult<KconfigInput, FunctionCall> {
    map(
        delimited(
            tag("$("),
            tuple((
                terminated(parse_function_name, opt(ws(tag(",")))),
                separated_list0(ws(tag(",")), ws(parse_parameter)),
            )),
            ws(tag(")")),
        ),
        |(name, parameters)| FunctionCall {
            name: name.to_string(),
            parameters,
        },
    )(input)
}

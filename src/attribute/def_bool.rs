use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_expression, parse_if_expression_attribute, Expression};

/// This is a shorthand notation for a bool type definition plus a value. Optionally dependencies for this default value can be added with "if".
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DefBool {
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

/// Parses a `def_bool` attribute.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///         def_bool::{parse_def_bool, DefBool},
///         expression::{Expression, AndExpression, Atom, OrExpression, Term},
///         function::{ExpressionToken, FunctionCall, Parameter},
///     },
///     symbol::Symbol,
/// };
///
/// assert_parsing_eq!(parse_def_bool, "def_bool     !PCI",  Ok(("", DefBool {
///     expression: Expression(OrExpression::Term(AndExpression::Term(Term::Not(
///         Atom::Symbol(Symbol::Constant("PCI".to_string()))
///     )))),
///     r#if: None
/// })));
/// ```
pub fn parse_def_bool(input: KconfigInput) -> IResult<KconfigInput, DefBool> {
    map(
        tuple((
            ws(tag("def_bool")),
            ws(parse_expression),
            opt(parse_if_expression_attribute),
        )),
        |(_, e, i)| DefBool {
            expression: e,
            r#if: i,
        },
    )(input)
}

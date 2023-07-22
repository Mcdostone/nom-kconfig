use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::wsi, KconfigInput};

use super::expression::{parse_expression, parse_if_expression_attribute, Expression};

/// This is a shorthand notation for a tristate type definition plus a value.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefTristate {
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

/// Parses a `def_tristate` attribute.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///         def_tristate::{parse_def_tristate, DefTristate},
///         expression::{Expression, AndExpression, Atom, OrExpression, Term},
///         function::{ExpressionToken, FunctionCall, Parameter},
///     },
///     symbol::Symbol,
/// };
///
/// assert_parsing_eq!(parse_def_tristate, "def_tristate PCI",  Ok(("", DefTristate {
///     expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
///         Atom::Symbol(Symbol::Constant("PCI".to_string()))
///     )))),
///     r#if: None
/// })));
/// ```
pub fn parse_def_tristate(input: KconfigInput) -> IResult<KconfigInput, DefTristate> {
    map(
        tuple((
            wsi(tag("def_tristate")),
            wsi(parse_expression),
            opt(parse_if_expression_attribute),
        )),
        |(_, e, i)| DefTristate {
            expression: e,
            r#if: i,
        },
    )(input)
}

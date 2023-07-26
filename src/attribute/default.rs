use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::{parse_expression, parse_if_expression_attribute, Expression};

/// A config option can have any number of default values. If multiple default values are visible, only the first defined one is active. Default values are not limited to the menu entry where they are defined. This means the default can be defined somewhere else or be overridden by an earlier definition. The default value is only assigned to the config symbol if no other value was set by the user (via the input prompt above).
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefaultAttribute {
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}

/// Parses a `default` attribute.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///         default::{parse_default, DefaultAttribute},
///         expression::{Expression, AndExpression, Atom, OrExpression, Term},
///         function::{ExpressionToken, FunctionCall, Parameter},
///     },
///     symbol::Symbol,
/// };
///
/// assert_parsing_eq!(
///     parse_default, "default 0x1",
///     Ok((
///         "",
///         DefaultAttribute {
///             expression: Expression::Term(AndExpression::Term(Term::Atom(
///                 Atom::Symbol(Symbol::Constant("0x1".to_string()))
///             ))),
///             r#if: None
///         }
///     ))
/// )
/// ```
pub fn parse_default(input: KconfigInput) -> IResult<KconfigInput, DefaultAttribute> {
    map(
        tuple((
            ws(tag("default")),
            ws(parse_expression),
            opt(parse_if_expression_attribute),
        )),
        |(_, e, i)| DefaultAttribute {
            expression: e,
            r#if: i,
        },
    )(input)
}

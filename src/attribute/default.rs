use nom::{bytes::complete::tag, combinator::map, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

use crate::{util::ws, KconfigInput};

use super::{parse_expression, parse_if_attribute, Expression};

/// A config option can have any number of default values.
/// If multiple default values are visible, only the first defined one is active.
/// Default values are not limited to the menu entry where they are defined.
/// This means the default can be defined somewhere else or be overridden by an earlier definition.
/// The default value is only assigned to the config symbol if no other value was set by the user.
///
/// see ["default value"](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes) for more information.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct DefaultAttribute {
    pub expression: Expression,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

#[cfg(feature = "display")]
impl Display for DefaultAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} if {}", self.expression, i),
            None => write!(f, "{}", self.expression),
        }
    }
}

/// Parses a `default` attribute.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///         parse_default, DefaultAttribute,
///         Expression, AndExpression, Atom, OrExpression, Term
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
        (ws(tag("default")), ws(parse_expression), parse_if_attribute),
        |(_, e, i)| DefaultAttribute {
            expression: e,
            r#if: i,
        },
    )
    .parse(input)
}

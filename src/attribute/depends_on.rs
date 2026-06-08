#[cfg(feature = "display")]
use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    IResult, Parser,
};

use super::expression::parse_expression;
use crate::{
    attribute::{expression::parse_if_expression, Expression},
    util::wsi,
    KconfigInput,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

/// While normal dependencies reduce the upper limit of a symbol, reverse dependencies can be used to force a lower limit of another symbol. The value of the current menu symbol is used as the minimal value [symbol](crate::Symbol) can be set to. If [symbol](crate::Symbol) is selected multiple times, the limit is set to the largest selection. Reverse dependencies can only be used with boolean or tristate symbols.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct DependsOn {
    pub expression: Expression,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub r#if: Option<Expression>,
}

/// Parses a `depends on` attribute.
/// If multiple dependencies are defined, they are connected with '&&'.
/// Dependencies are applied to all other options within this menu entry (which also accept an "if" expression).
/// See [https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes) for more information.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///         parse_depends_on,
///         AndExpression, Atom, Expression, OrExpression, Term,
///     },
///     symbol::Symbol,
///     Attribute
/// };
///
/// assert_parsing_eq!(
///     parse_depends_on,
///     "depends on PCI",
///     Ok((
///         "",
///         DependsOn { expression: Expression::Term(AndExpression::Term(
///             Term::Atom(Atom::Symbol(Symbol::NonConstant("PCI".to_string())))
///         )), r#if: None }
///     ))
/// )
/// ```
pub fn parse_depends_on(input: KconfigInput) -> IResult<KconfigInput, DependsOn> {
    map(
        (
            tag("depends"),
            wsi(opt(tag("on"))),
            wsi(parse_expression),
            opt(parse_if_expression),
        ),
        |(_, _, e, r#if)| DependsOn {
            expression: e,
            r#if,
        },
    )
    .parse(input)
}

#[cfg(feature = "display")]
impl Display for DependsOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.r#if {
            Some(i) => write!(f, "{} if {}", self.expression, i),
            None => write!(f, "{}", self.expression),
        }
    }
}

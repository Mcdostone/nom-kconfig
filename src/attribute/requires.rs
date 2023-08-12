use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_expression, Expression};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Requires {
    pub symbol: Expression,
}
/// Parses a `requires` attribute.
/// TODO: I think this attribute is deprecated.
///
/// /// Parses a `range` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///     parse_requires, Requires,
///     AndExpression, Atom, CompareExpression, CompareOperator, Expression, OrExpression, Term},
///     symbol::Symbol,
/// };
///
/// assert_parsing_eq!(
///     parse_requires,
///     " requires  KVM",
///     Ok((
///         "",
///         Requires {
///             symbol: Expression::Term(AndExpression::Term(Term::Atom(
///                    Atom::Symbol(Symbol::Constant("KVM".to_string())
///                ))))
///         }
///     ))
/// )
/// ```
pub fn parse_requires(input: KconfigInput) -> IResult<KconfigInput, Requires> {
    map(
        tuple((ws(tag("requires")), ws(parse_expression))),
        |(_, s)| Requires { symbol: s },
    )(input)
}

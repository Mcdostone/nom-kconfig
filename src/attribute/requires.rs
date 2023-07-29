use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::{Deserialize, Serialize};

use crate::{util::ws, KconfigInput};

use super::expression::{parse_expression, Expression};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
///     requires::{parse_requires, Requires},
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

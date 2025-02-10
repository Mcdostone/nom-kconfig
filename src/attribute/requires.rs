use nom::{bytes::complete::tag, sequence::preceded, IResult, Parser};

use super::expression::{parse_expression, Expression};
use crate::{util::ws, KconfigInput};

/// Parses a `requires` attribute.
/// TODO: I think this attribute is deprecated.
///
/// /// Parses a `range` attribute.
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///     parse_requires,
///     AndExpression, Atom, CompareExpression, CompareOperator, Expression, OrExpression, Term},
///     symbol::Symbol,
/// };
///
/// assert_parsing_eq!(
///     parse_requires,
///     " requires  KVM",
///     Ok((
///         "",
///         Expression::Term(AndExpression::Term(Term::Atom(
///             Atom::Symbol(Symbol::Constant("KVM".to_string())
///         ))))
///     ))
/// )
/// ```
pub fn parse_requires(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    preceded(ws(tag("requires")), ws(parse_expression)).parse(input)
}

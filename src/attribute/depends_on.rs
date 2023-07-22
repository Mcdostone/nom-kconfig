use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use crate::{util::ws, KconfigInput};

use super::{expression::parse_expression, Attribute};

/// Parses a `depends on` attribute.
/// If multiple dependencies are defined, they are connected with '&&'. Dependencies are applied to all other options within this menu entry (which also accept an "if" expression), so these two examples are equivalent:
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::{
///         depends_on::parse_depends_on,
///         expression::{AndExpression, Atom, Expression, OrExpression, Term},
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
///         Attribute::DependsOn(Expression(OrExpression::Term(AndExpression::Term(
///             Term::Atom(Atom::Symbol(Symbol::Constant("PCI".to_string())))
///         ))))
///     ))
/// )
/// ```
pub fn parse_depends_on(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    map(
        tuple((tag("depends"), ws(opt(tag("on"))), ws(parse_expression))),
        |(_, _, e)| Attribute::DependsOn(e),
    )(input)
}

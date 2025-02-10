use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    IResult, Parser,
};

use crate::{util::wsi, KconfigInput};

use super::{expression::parse_expression, Attribute};

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
///         Attribute::DependsOn(Expression::Term(AndExpression::Term(
///             Term::Atom(Atom::Symbol(Symbol::Constant("PCI".to_string())))
///         )))
///     ))
/// )
/// ```
pub fn parse_depends_on(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    map(
        (tag("depends"), wsi(opt(tag("on"))), wsi(parse_expression)),
        |(_, _, e)| Attribute::DependsOn(e),
    )
    .parse(input)
}

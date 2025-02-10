use nom::{
    bytes::complete::tag,
    combinator::{cut, map},
    multi::many0,
    sequence::{pair, terminated},
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    attribute::expression::{parse_if_expression, Expression},
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry};

/// This defines an if block. The dependency expression [expr]((crate::attribute::expression)) is appended to all enclosed menu entries.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct If {
    pub condition: Expression,
    pub entries: Vec<Entry>,
}
/// it parses a if block.
///
/// # Example
/// ```
///  use nom_kconfig::{
/// assert_parsing_eq,
/// Symbol,
/// entry::{Entry, parse_if, Comment, If},
/// attribute::{Term, Expression, AndExpression, Atom}
/// };
/// assert_parsing_eq!(
///     parse_if,
///     r#"if NET_VENDOR_AMD comment "Support of PCI" endif"#,
///     Ok((
///         "",
///         If {
///             condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
///                 Symbol::Constant("NET_VENDOR_AMD".to_string())
///             )))),
///             entries: vec!(Entry::Comment(Comment { prompt: "Support of PCI".to_string(), dependencies: vec!() }))
///         }
///     ))
/// )
/// ```
pub fn parse_if(input: KconfigInput) -> IResult<KconfigInput, If> {
    map(
        pair(
            ws(parse_if_expression),
            cut(terminated(many0(parse_entry), ws(tag("endif")))),
        ),
        |(condition, entries)| If { condition, entries },
    )
    .parse(input)
}

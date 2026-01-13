use nom::{bytes::complete::tag, combinator::value, IResult, Parser};

use crate::{util::ws, KconfigInput};

/// A transitional symbol meansit should be processed during configuration but omitted from newly written .config files. Transitional symbols are useful for backward compatibility during config option migrations - they allow olddefconfig to process existing .config files while ensuring the old option doesn't appear in new configurations.
///
/// # Example
/// ```
/// use nom_kconfig::{assert_parsing_eq, attribute::transitional::parse_transitional};
///
/// assert_parsing_eq!(parse_transitional, "transitional", Ok(("", ())))
/// ```
pub fn parse_transitional(input: KconfigInput) -> IResult<KconfigInput, ()> {
    value((), ws(tag("transitional"))).parse(input)
}

use nom::{bytes::complete::tag, combinator::value, IResult, Parser};

use crate::{util::ws, KconfigInput};

/// The "modules" attribute declares the symbol to be used as the MODULES symbol, which enables the third modular state for all config symbols.
/// At most one symbol may have the "modules" option set.
///
/// # Example
/// ```
/// use nom_kconfig::{assert_parsing_eq, attribute::parse_modules};
/// assert_parsing_eq!(parse_modules, "modules", Ok(("", ())))
/// ```
pub fn parse_modules(input: KconfigInput) -> IResult<KconfigInput, ()> {
    value((), ws(tag("modules"))).parse(input)
}

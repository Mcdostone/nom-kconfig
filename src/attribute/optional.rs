use nom::{bytes::complete::tag, combinator::value, IResult, Parser};

use crate::{util::ws, KconfigInput};

/// A choice entry accepts the attribute [Optional](crate::attribute::optional), which allows to set the choice to 'n' and no entry needs to be selected. If no [symbol](crate::symbol) is associated with a choice, then you can not have multiple definitions of that choice. If a [Symbol](crate::symbol) is associated to the choice, then you may define the same choice (i.e. with the same entries) in another place.
///
/// # Example
/// ```
/// use nom_kconfig::{assert_parsing_eq, attribute::parse_optional};
///
/// assert_parsing_eq!(parse_optional, "optional", Ok(("", ())))
/// ```
pub fn parse_optional(input: KconfigInput) -> IResult<KconfigInput, ()> {
    value((), ws(tag("optional"))).parse(input)
}

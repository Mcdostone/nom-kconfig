use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

/// Parses a `enable` attribute. It looks like this attribute is deprecated....
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::parse_enable,
/// };
///
/// assert_parsing_eq!(
///     parse_enable,
///     "enable MTK_INFRACFG",
///     Ok((
///         "",
///         "MTK_INFRACFG".to_string()
///     ))
/// )
/// ```
pub fn parse_enable(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        tuple((ws(tag("enable")), ws(parse_constant_symbol))),
        |(_, s)| s.to_string(),
    )(input)
}

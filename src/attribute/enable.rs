use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

/// Parses a `enable` attribute. I was not able to find documentation regarding this attribute.
/// I think it is deprecated.
/// This attribute has been found in this file [v2.6.1/drivers/net/wireless/Kconfig](https://cdn.kernel.org/pub/linux/kernel/v2.6/linux-2.6.1.tar.xz).
/// `enable` has been replaced with `select`.
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

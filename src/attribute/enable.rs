use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, KconfigInput};

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Enable {
    pub symbol: String,
}

/// Parses a `enable` attribute. It looks like this attribute is deprecated....
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::enable::{parse_enable, Enable},
/// };
///
/// assert_parsing_eq!(
///     parse_enable,
///     "enable MTK_INFRACFG",
///     Ok((
///         "",
///         Enable {
///             symbol: "MTK_INFRACFG".to_string()
///         }
///     ))
/// )
/// ```
pub fn parse_enable(input: KconfigInput) -> IResult<KconfigInput, Enable> {
    map(
        tuple((ws(tag("enable")), ws(parse_constant_symbol))),
        |(_, s)| Enable {
            symbol: s.to_string(),
        },
    )(input)
}

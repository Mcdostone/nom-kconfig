/// An rsource statement is available for including files specified with a relative path.
/// The path is relative to the directory of the Kconfig file that contains the rsource statement.
/// <https://docs.zephyrproject.org/latest/build/kconfig/extensions.html>
use nom::{bytes::complete::tag, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{entry::Source, util::ws, KconfigInput};


pub type OrSource = Source;


#[allow(dead_code)]
pub fn parse_orsource(input: KconfigInput) -> IResult<KconfigInput, ORSource> {
    let (input, _) = ws(tag("orsource")).parse(input)?;
    Ok((input, RSource(Source { entries: vec![] })))
}

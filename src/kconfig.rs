use nom::{
    combinator::{eof, map},
    multi::many0,
    sequence::delimited,
    IResult,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    entry::{parse_entry, Entry},
    util::{ws, ws_comment},
    KconfigInput,
};

/// A Kconfig file.
/// Field `file` is relative to the root directory defined in [KconfigFile](crate::KconfigFile).
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Kconfig<'a> {
    pub file: String,
    #[cfg_attr(feature = "deserialize", serde(borrow))]
    pub entries: Vec<Entry<'a>>,
    pub input: &'a str,
}

/// Parses a kconfig input.
/// # Example
/// ```
/// use std::path::PathBuf;
/// use nom_kconfig::{KconfigInput, KconfigFile, Entry, kconfig::parse_kconfig, Kconfig};
///
/// let kconfig_file = KconfigFile::new(PathBuf::from("path/to/root/dir"), PathBuf::from("Kconfig"), "".to_string());
/// let content = "";
/// let input = KconfigInput::new_extra(content, kconfig_file);
/// assert_eq!(parse_kconfig(input).unwrap().1, Kconfig {file: "Kconfig".to_string(), entries: vec!() })
/// ```
pub fn parse_kconfig(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, Kconfig<'_>> {
    let file: std::path::PathBuf = input.extra.file.clone();
    let i = input.fragment();
    let (input, result) = map(delimited(ws_comment, many0(parse_entry), ws(eof)), |d| {
        Kconfig {
            file: file.display().to_string(),
            entries: d,
            input: i
        }
    })(input)?;
    Ok((input, result))
}

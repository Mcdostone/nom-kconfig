use nom::{
    combinator::{eof, map},
    multi::many0,
    sequence::delimited,
    IResult,
};
use serde::Serialize;

use crate::{
    entry::{parse_entry, Entry},
    util::{ws, ws_comment},
    KconfigInput,
};

/// A Kconfig file.
/// `file` is always relative to the root directory defined in [KconfigFile](crate::KconfigFile)
#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Kconfig {
    pub file: String,
    pub entries: Vec<Entry>,
}

/// Parses a kconfig input.
/// # Example
/// ```
/// use std::path::PathBuf;
/// use nom_kconfig::{KconfigInput, KconfigFile, Entry, kconfig::parse_kconfig, Kconfig};
///
/// let kconfig_file = KconfigFile::new(PathBuf::from("path/to/root/dir"), PathBuf::from("Kconfig"));
/// let content = "";
/// let input = KconfigInput::new_extra(content, kconfig_file);
/// assert_eq!(parse_kconfig(input).unwrap().1, Kconfig {file: "Kconfig".to_string(), entries: vec!() })
/// ```
pub fn parse_kconfig(input: KconfigInput) -> IResult<KconfigInput, Kconfig> {
    let file: std::path::PathBuf = input.extra.file.clone();
    let (input, result) = map(delimited(ws_comment, many0(parse_entry), ws(eof)), |d| {
        Kconfig {
            file: file.display().to_string(),
            entries: d,
        }
    })(input)?;
    Ok((input, result))
}

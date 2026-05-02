use std::collections::HashMap;

use nom::{
    combinator::{eof, map},
    multi::many0,
    sequence::delimited,
    IResult, Parser,
};
use regex::Regex;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use tracing::debug;

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
    debug!("parsing '{}'", input.extra.full_path().display());
    //let prefix = (0..input.extra.depth).map(|_| "   ").collect::<String>();
    //debug!("{} {}", prefix, input.extra.full_path().display());
    let file: std::path::PathBuf = input.extra.file.clone();

    let preprocessed_content = preprocess_macros(input.fragment(), input.extra.vars());
    let _lol = KconfigInput::new_extra(&preprocessed_content, input.extra.clone());

    let (input, result) = map(delimited(ws_comment, many0(parse_entry), ws(eof)), |d| {
        Kconfig {
            file: file.display().to_string(),
            entries: d,
        }
    })
    .parse(input)?;
    Ok((input, result))
}

pub fn preprocess_macros(content: &str, extra_vars: &HashMap<String, String>) -> String {
    let re = Regex::new(r"\$\((\S+)\)").unwrap();
    let mut file_copy = String::from(content);
    for (var_name, var_value) in re.captures_iter(content).map(|cap| {
        let ex: (&str, [&str; 1]) = cap.extract();
        let var = ex.1[0];
        (var, extra_vars.get(var))
    }) {
        if let Some(var_value) = var_value {
            file_copy = file_copy.replace(&format!("$({var_name})"), var_value);
        }
    }

    let re = Regex::new(r"\$\{(\S+)\}").unwrap();
    let mut file_copy = String::from(content);
    for (var_name, var_value) in re.captures_iter(content).map(|cap| {
        let ex: (&str, [&str; 1]) = cap.extract();
        let var = ex.1[0];
        (var, extra_vars.get(var))
    }) {
        if let Some(var_value) = var_value {
            file_copy = file_copy.replace(&format!("${{{var_name}}}"), var_value);
        }
    }

    file_copy
}

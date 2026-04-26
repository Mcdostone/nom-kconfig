#[cfg(feature = "kconfiglib")]
mod osource;
#[cfg(feature = "kconfiglib")]
mod rsource;
#[cfg(feature = "kconfiglib")]
mod orsource;
#[allow(clippy::module_inception)]
mod source;

use nom::{
    branch::alt,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    IResult, Parser,
};

use regex::Regex;
pub use source::{parse_source, Source};

#[cfg(feature = "kconfiglib")]
pub use self::{
    osource::parse_osource, rsource::parse_rsource, orsource::parse_orsource,
    osource::OSource, rsource::RSource, orsource::OrSource
};

#[cfg(any(feature = "kconfiglib"))]
use crate::{parse_kconfig, util::ws, Kconfig, KconfigFile, KconfigInput};

#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
pub use glob::glob;
#[cfg(feature = "kconfiglib")]

use std::collections::HashMap;
#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
use std::path::PathBuf;

#[cfg(test)]
mod source_test;



enum JoinPathMode {
    Relative,
    Root,
}


pub(crate) fn parse_filepath(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, &str> {
    map(
        recognize(ws(many1(alt((
            alphanumeric1::<KconfigInput, _>,
            recognize(one_of(".$()*-_$+@/")),
        ))))),
        |d| d.fragment().to_owned(),
    )
    .parse(input)
}

fn parse_source_kconfig(
    input: KconfigInput,
    source_kconfig_file: KconfigFile,
) -> Result<Kconfig, nom::Err<Error<KconfigInput>>> {
    let source_content = source_kconfig_file
        .read_to_string()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;

    #[allow(clippy::let_and_return)]
    let x = match cut(parse_kconfig).parse(KconfigInput::new_extra(
        &source_content,
        source_kconfig_file.clone(),
    )) {
        Ok((_, kconfig)) => Ok(kconfig),
        Err(_e) => Err(nom::Err::Error(Error::new(
            KconfigInput::new_extra("", source_kconfig_file),
            ErrorKind::Fail,
        ))),
    };
    x
}



pub fn apply_vars(
    file: &str,
    extra_vars: &HashMap<String, String>,
) -> Option<String> {
    let re = Regex::new(r"\$\((\S+)\)").unwrap();
    let mut file_copy = String::from(file);
    for (var_name, var_value) in re.captures_iter(file).map(|cap| {
        let ex: (&str, [&str; 1]) = cap.extract();
        let var = ex.1[0];
        (var, extra_vars.get(var))
    }) {
        if let Some(var_value) = var_value {
            file_copy = file_copy.replace(&format!("$({var_name})"), var_value);
        } else {
            return None;
        }
    }
    Some(file_copy)
}

#[cfg(any(feature = "kconfiglib", feature = "coreboot"))]
fn expand_source_files<'a>(
    input: KconfigInput<'a>,
    file: &str,
    mode: JoinPathMode
) -> Result<Vec<PathBuf>, nom::Err<Error<KconfigInput<'a>>>> {
    let full_path_pattern = input.extra.root_dir.join(file).display().to_string();
    let mut expanded_files = Vec::new();

    let prefix_path = match mode {
        JoinPathMode::Relative =>  PathBuf::from(file).parent().unwrap().to_path_buf(),
        JoinPathMode::Root => input.extra.root_dir.clone(),
    };

    let paths: Vec<PathBuf> = glob(&full_path_pattern)
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))
        ?;

    if paths.is_empty() {
        return Ok(vec![prefix_path.join(file)]);
    }
    for source_path in paths {
        let source_path_without_root = source_path
            .strip_prefix(&prefix_path)
            .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;
        expanded_files.push(source_path_without_root.to_path_buf());
    }
    
    expanded_files.sort();
    if expanded_files.is_empty() {
        return Err(nom::Err::Error(Error::from_error_kind(
            input,
            ErrorKind::Fail,
        )));
    }

    Ok(expanded_files)
}






#[cfg(test)]
use crate::assert_parsing_eq;

#[test]
fn test_parse_filepath() {
    assert_parsing_eq!(
        parse_filepath,
        "u-boot/board/sagem/f@st1704/Kconfig",
        Ok(("", "u-boot/board/sagem/f@st1704/Kconfig"))
    );

    assert_parsing_eq!(
        parse_filepath,
        "u-boot/board/l+g/vinco/Kconfig",
        Ok(("", "u-boot/board/l+g/vinco/Kconfig"))
    );
}
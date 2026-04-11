use std::path::PathBuf;

#[cfg(feature = "coreboot")]
use glob::glob;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    sequence::delimited,
    IResult, Parser,
};
use regex::Regex;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    kconfig::{parse_kconfig, Kconfig},
    util::{ws, wsi},
    KconfigFile, KconfigInput,
};

pub fn parse_filepath(input: KconfigInput<'_>) -> IResult<KconfigInput<'_>, &str> {
    map(
        recognize(ws(many1(alt((
            alphanumeric1::<KconfigInput, _>,
            recognize(one_of(".$()*-_$/")),
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

#[cfg(feature = "coreboot")]
fn expand_source_files<'a>(
    input: KconfigInput<'a>,
    file: &str,
) -> Result<Vec<PathBuf>, nom::Err<Error<KconfigInput<'a>>>> {
    let full_path_pattern = input.extra.root_dir.join(file).display().to_string();
    let mut expanded_files = Vec::new();
    for source_path in glob(&full_path_pattern)
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?
    {
        let source_path = source_path
            .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;
        let source_path_without_root = source_path
            .strip_prefix(&input.extra.root_dir)
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

pub fn parse_source(input: KconfigInput) -> IResult<KconfigInput, Source> {
    let (input, _) = ws(tag("source")).parse(input)?;
    let (input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;
    if let Some(file) = apply_vars(file, &input.extra.vars) {
        #[cfg(feature = "coreboot")]
        {
            let expanded_files = expand_source_files(input.clone(), &file)?;
            let mut sources = vec![];

            for expanded_file in expanded_files {
                let source_kconfig_file = KconfigFile::new_with_vars(
                    input.clone().extra.root_dir,
                    expanded_file,
                    &input.extra.vars,
                );
                let source = parse_source_kconfig(input.clone(), source_kconfig_file)?;
                sources.push(source);
            }

            Ok((input, Source { entries: sources }))
        }

        #[cfg(not(feature = "coreboot"))]
        {
            let source_kconfig_file = KconfigFile::new_with_vars(
                input.clone().extra.root_dir,
                PathBuf::from(file),
                &input.extra.vars,
            );
            let source = parse_source_kconfig(input.clone(), source_kconfig_file)?;
            Ok((
                input,
                Source {
                    entries: vec![source],
                },
            ))
        }
    } else {
        Ok((
            input,
            Source {
                entries: vec![Kconfig {
                    file: file.to_string(),
                    ..Default::default()
                }],
            },
        ))
    }
}

pub fn apply_vars(
    file: &str,
    extra_vars: &std::collections::HashMap<String, String>,
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

/// Entry that reads the specified configuration file. This file is always parsed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Source {
    pub entries: Vec<Kconfig>,
}

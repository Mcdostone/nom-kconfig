use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::ErrorKind,
    multi::many1,
    sequence::delimited,
    IResult,
};
use regex::Regex;

use crate::{
    kconfig::{parse_kconfig, Kconfig},
    util::{ws, wsi},
    KconfigFile, KconfigInput,
};

pub fn parse_filepath(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        recognize(ws(many1(alt((
            alphanumeric1::<KconfigInput, _>,
            recognize(one_of(".$()-_$/")),
        ))))),
        |d| d.fragment().to_owned(),
    )(input)
}

pub fn parse_source(input: KconfigInput) -> IResult<KconfigInput, Source> {
    let (input, _) = ws(tag("source"))(input)?;
    let (input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))(input)?;
    let source_kconfig_file = KconfigFile::new(input.clone().extra.root_dir, PathBuf::from(file));
    if is_dynamic_source(file) {
        return Ok((
            input,
            Source {
                file: file.to_string(),
                ..Default::default()
            },
        ));
    }
    if let Ok(ff) = source_kconfig_file.read_to_string() {
        return match cut(parse_kconfig)(KconfigInput::new_extra(
            ff.as_str(),
            source_kconfig_file.clone(),
        )) {
            Ok((_, kconfig)) => Ok((input, kconfig)),
            Err(_err) => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    KconfigInput::new_extra("", source_kconfig_file),
                    ErrorKind::Fail,
                )))
            }
        };
    }
    Err(nom::Err::Error(nom::error::Error::new(
        KconfigInput::new_extra("", source_kconfig_file),
        ErrorKind::Fail,
    )))
}

fn is_dynamic_source(file: &str) -> bool {
    let re = Regex::new("\\$(.+)").unwrap();
    re.is_match(file)
}

/// Entry that reads the specified configuration file. This file is always parsed.
pub type Source = Kconfig;

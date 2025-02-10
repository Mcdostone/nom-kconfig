use std::path::PathBuf;

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
    )
    .parse(input)
}

pub fn parse_source(input: KconfigInput) -> IResult<KconfigInput, Source> {
    let (input, _) = ws(tag("source")).parse(input)?;
    let (input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;
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
    let source_content = source_kconfig_file
        .read_to_string()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;

    let binding = source_content.clone();
    #[allow(clippy::let_and_return)]
    let x = match cut(parse_kconfig).parse(KconfigInput::new_extra(
        &binding,
        source_kconfig_file.clone(),
    )) {
        Ok((_, kconfig)) => Ok((input, kconfig)),
        Err(_e) => Err(nom::Err::Error(nom::error::Error::new(
            KconfigInput::new_extra("", source_kconfig_file),
            ErrorKind::Fail,
        ))),
    };
    x
}

fn is_dynamic_source(file: &str) -> bool {
    let re = Regex::new("\\$(.+)").unwrap();
    re.is_match(file)
}

/// Entry that reads the specified configuration file. This file is always parsed.
pub type Source = Kconfig;

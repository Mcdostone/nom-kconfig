use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    sequence::delimited,
    IResult,
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

    let source_kconfig_file = KconfigFile::new(
        input.clone().extra.root_dir,
        PathBuf::from(file)
    );
    if is_dynamic_source(file) {
        return Ok((
            input,
            Source {
                content: "".to_string().into(),
                kconfig: Kconfig {
                    file: file.to_string(),
                    ..Default::default()
                },
            },
        ));
    }
    

    let mut source = Source {
        content: Box::new("".to_string()),
        kconfig: Kconfig {
            file: file.to_string(),
            entries: vec![],
            input: ""
        },
    };

    let content =
        Box::new(source_kconfig_file.read_to_string().map_err(|_| {
            nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail))
        })?);
    let k = KconfigInput::new_extra(&content, source_kconfig_file.clone());

    let x = match cut(parse_kconfig)(k) {
        Ok((_, kconfig)) => {
            source.kconfig = kconfig;
            return Ok((
                "".into(),
                Source {
                    content: "".to_string().into(),
                    kconfig: Kconfig {
                        file: file.to_string(),
                        entries: vec![],
                        input: ""
                    },
                },
            ));
        }
        Err(_e) => Err(nom::Err::Error(nom::error::Error::new(
            KconfigInput::new_extra("", source_kconfig_file),
            ErrorKind::Fail,
        ))),
    };

    #[allow(clippy::let_and_return)]
    x
}

fn is_dynamic_source(file: &str) -> bool {
    let re = Regex::new("\\$(.+)").unwrap();
    re.is_match(file)
}

/// Entry that reads the specified configuration file. This file is always parsed.
/// #[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "hash", derive(Hash))]
#[derive(PartialEq, Clone, Debug)]
pub struct Source<'a> {
    pub content: Box<String>,
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub kconfig: Kconfig<'a>,
}

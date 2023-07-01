use std::path::PathBuf;

use nom::{bytes::complete::tag, IResult};
use regex::Regex;

use crate::{
    attribute::prompt::parse_prompt_option,
    kconfig::{parse_kconfig, Kconfig},
    util::ws,
    KconfigFile, KconfigInput,
};

pub fn parse_source(input: KconfigInput) -> IResult<KconfigInput, Source> {
    let (input, _) = ws(tag("source"))(input)?;
    let (input, file) = ws(parse_prompt_option)(input)?;
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
        return match parse_kconfig(KconfigInput::new_extra(&ff, source_kconfig_file.clone())) {
            Ok((_, kconfig)) => {
                //      println!("{:?}", kconfig);
                Ok((input, kconfig))
            }
            Err(_err) => Err(nom::Err::Error(nom::error::Error::new(
                KconfigInput::new_extra("", source_kconfig_file),
                nom::error::ErrorKind::Fail,
            ))),
        };
    }
    Err(nom::Err::Error(nom::error::Error::new(
        KconfigInput::new_extra("", source_kconfig_file),
        nom::error::ErrorKind::Fail,
    )))
}

fn is_dynamic_source(file: &str) -> bool {
    let re = Regex::new("$(.+)").unwrap();
    re.is_match(file)
}

pub type Source = Kconfig;

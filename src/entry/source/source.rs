use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(any(feature = "coreboot", feature = "kconfiglib"))]
use crate::entry::source::{expand_source_files, JoinPathMode};
use crate::{
    entry::source::{apply_vars, parse_filepath, parse_source_kconfig},
    kconfig::Kconfig,
    util::{ws, wsi},
    KconfigFile, KconfigInput,
};

/// Entry that reads the specified configuration file. This file is always parsed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Source {
    pub kconfigs: Vec<Kconfig>,
}

pub fn parse_source(input: KconfigInput) -> IResult<KconfigInput, Source> {
    let (input, _) = ws(tag("source")).parse(input)?;
    let (input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;
    if let Some(file) = apply_vars(file, &input.extra.vars()) {
        #[cfg(any(feature = "coreboot", feature = "kconfiglib"))]
        {
            let expanded_files = expand_source_files(input.clone(), &file, JoinPathMode::Root)?;
            let mut sources = vec![];

            for expanded_file in expanded_files {
                let source_kconfig_file = KconfigFile::new_with_vars(
                    input.clone().extra.root_dir,
                    expanded_file,
                    &input.extra.vars(),
                );
                let source = parse_source_kconfig(input.clone(), source_kconfig_file)?;
                sources.push(source);
            }

            Ok((input, Source { kconfigs: sources }))
        }

        #[cfg(not(any(feature = "coreboot", feature = "kconfiglib")))]
        {
            use std::path::PathBuf;

            let source_kconfig_file = KconfigFile::new_with_vars(
                input.clone().extra.root_dir,
                PathBuf::from(file),
                &input.extra.vars(),
            );
            let source = parse_source_kconfig(input.clone(), source_kconfig_file)?;
            Ok((
                input,
                Source {
                    kconfigs: vec![source],
                },
            ))
        }
    } else {
        Ok((
            input,
            Source {
                kconfigs: vec![Kconfig {
                    file: file.to_string(),
                    ..Default::default()
                }],
            },
        ))
    }
}

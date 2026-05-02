use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(any(feature = "coreboot", feature = "kconfiglib"))]
use crate::entry::source::{expand_source_files, JoinPathMode};
use crate::{
    entry::source::{parse_filepath, parse_source_kconfig},
    kconfig::Kconfig,
    util::{ws, wsi},
    KconfigInput,
};
use tracing::span;

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
    let (mut input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;

    #[cfg(any(feature = "coreboot", feature = "kconfiglib"))]
    {
        let expanded_files = expand_source_files(input.clone(), file, JoinPathMode::Root)?;
        let mut sources = vec![];

        for expanded_file in expanded_files {
            use tracing::Level;

            let my_span = span!(Level::TRACE, "parsing-source", variables = ?input.extra.vars());
            let _ = my_span.enter();
            let source_kconfig_file = input.extra.new_source_file(expanded_file);
            let (variables, source) = parse_source_kconfig(input.clone(), source_kconfig_file)?;
            input.extra.add_local_vars(variables);
            sources.push(source);
        }

        Ok((input, Source { kconfigs: sources }))
    }

    #[cfg(not(any(feature = "coreboot", feature = "kconfiglib")))]
    {
        use std::path::PathBuf;

        let source_kconfig_file = input.extra.new_source_file(PathBuf::from(file));
        let (variables, source) = parse_source_kconfig(input.clone(), source_kconfig_file)?;
        input.extra.add_local_vars(variables);
        return Ok((
            input,
            Source {
                kconfigs: vec![source],
            },
        ));
    }
}

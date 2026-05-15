/// An rsource statement is available for including files specified with a relative path.
/// The path is relative to the directory of the Kconfig file that contains the rsource statement.
/// <https://docs.zephyrproject.org/latest/build/kconfig/extensions.html>
use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};

use crate::{
    entry::{
        source::{expand_source_files, parse_filepath, parse_source_kconfig, JoinPathMode},
        Source,
    },
    kconfig::Kconfig,
    util::{ws, wsi},
    KconfigInput,
};

pub type OrSource = Source;

pub fn parse_orsource(input: KconfigInput) -> IResult<KconfigInput, OrSource> {
    let (input, _) = ws(tag("orsource")).parse(input)?;
    let (mut input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;
    let expanded_files = expand_source_files(input.clone(), file, JoinPathMode::Relative)?;
    let mut sources = vec![];

    for expanded_file in expanded_files {
        let source_kconfig_file = input.extra.new_source_file(expanded_file);
        if !source_kconfig_file.full_path().exists() {
            sources.push(Kconfig {
                file: file.to_string(),
                ..Default::default()
            });
            continue;
        }
        let (variables, source) = parse_source_kconfig(input.clone(), source_kconfig_file)?;
        input.extra.add_local_vars(variables);
        sources.push(source);
    }

    Ok((input, OrSource { kconfigs: sources }))
}

use crate::entry::source::JoinPathMode;
use crate::{entry::Source, util::ws, KconfigInput};
use nom::{bytes::complete::tag, IResult, Parser};

use nom::{branch::alt, sequence::delimited};

use crate::{
    entry::source::{apply_vars, expand_source_files, parse_filepath, parse_source_kconfig},
    util::wsi,
    KconfigFile,
};

pub type RSource = Source;

#[allow(dead_code)]
pub fn parse_rsource(input: KconfigInput) -> IResult<KconfigInput, RSource> {
    let (input, _) = ws(tag("rsource")).parse(input)?;
    let (input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;

    let file = apply_vars(file, &input.extra.vars()).unwrap_or(file.to_string());
    let expanded_files = expand_source_files(input.clone(), &file, JoinPathMode::Relative)?;
    let mut sources = vec![];
    dbg!(&expanded_files);
    for expanded_file in expanded_files {
        let source_kconfig_file = KconfigFile::new_with_vars(
            input.clone().extra.root_dir,
            expanded_file,
            &input.extra.vars(),
        );

        let source = parse_source_kconfig(input.clone(), source_kconfig_file)?;
        sources.push(source);
    }

    Ok((input, RSource { kconfigs: sources }))
}

#[cfg(test)]
use std::path::PathBuf;

#[test]
#[ignore]
fn test_parse_rsource() {
    let res = parse_rsource(KconfigInput::new_extra(
        r#"rsource "boards/*.defconfig""#,
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

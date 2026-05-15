/// For cases where it's okay for a pattern to match no files (or for a plain filename to not exist), a separate osource (optional source) statement is available.
/// osource is a no-op if no file matches.
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

pub type OSource = Source;

pub fn parse_osource(input: KconfigInput) -> IResult<KconfigInput, OSource> {
    let (input, _) = ws(tag("osource")).parse(input)?;
    let (mut input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;
    let expanded_files = expand_source_files(input.clone(), file, JoinPathMode::Root)?;
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

    Ok((input, OSource { kconfigs: sources }))
}

#[cfg(test)]
use crate::attribute::r#type::ConfigType;
#[cfg(test)]
use crate::attribute::r#type::Type;
#[cfg(test)]
use crate::entry::config::Config;
#[cfg(test)]
use crate::Attribute;
#[cfg(test)]
use crate::Entry;
#[cfg(test)]
use std::path::PathBuf;

#[test]
fn test_parse_osource() {
    assert_parsing_osource_eq(
        r#"osource "Kconfig.simple""#,
        Ok((
            "",
            OSource {
                kconfigs: vec![Kconfig {
                    file: "Kconfig.simple".to_string(),
                    entries: vec![Entry::Config(Config {
                        symbol: "KVM".to_string(),
                        attributes: vec![Attribute::Type(ConfigType {
                            r#type: Type::Tristate(None),
                            r#if: None,
                        })],
                    })],
                }],
            },
        )),
    )
}

#[test]
fn test_parse_osource_does_not_exist() {
    assert_parsing_osource_eq(
        r#"osource "this-file-does-not-exist""#,
        Ok((
            "",
            OSource {
                kconfigs: vec![Kconfig {
                    file: "this-file-does-not-exist".to_string(),
                    ..Default::default()
                }],
            },
        )),
    )
}

#[cfg(test)]
fn assert_parsing_osource_eq(
    input: &str,
    expected: Result<(&str, OSource), nom::Err<nom::error::Error<KconfigInput>>>,
) {
    use crate::KconfigFile;

    let res = parse_osource(KconfigInput::new_extra(
        input,
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ))
    .map(|r| (r.0.fragment().to_owned(), r.1));
    assert_eq!(res, expected)
}

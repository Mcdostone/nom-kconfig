/// For cases where it's okay for a pattern to match no files (or for a plain filename to not exist), a separate osource (optional source) statement is available.
/// osource is a no-op if no file matches.
/// <https://docs.zephyrproject.org/latest/build/kconfig/extensions.html>
use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};

use crate::{
    entry::{
        source::{apply_vars, expand_source_files, parse_filepath, parse_source_kconfig},
        Source,
    },
    kconfig::Kconfig,
    util::{ws, wsi},
    KconfigFile, KconfigInput,
};

pub type OSource = Source;

pub fn parse_osource(input: KconfigInput) -> IResult<KconfigInput, OSource> {
    let (input, _) = ws(tag("osource")).parse(input)?;
    let (input, file) = wsi(alt((
        delimited(tag("\""), parse_filepath, tag("\"")),
        parse_filepath,
    )))
    .parse(input)?;
    if let Some(file) = apply_vars(file, &input.extra.vars) {
        let expanded_files = expand_source_files(input.clone(), &file)?;
        let mut sources = vec![];

        for expanded_file in expanded_files {
            let source_kconfig_file = KconfigFile::new_with_vars(
                input.clone().extra.root_dir,
                expanded_file,
                &input.extra.vars,
            );

            println!(
                "Checking if file exists: {} {}",
                source_kconfig_file.full_path().display(),
                source_kconfig_file.full_path().exists()
            );

            if !source_kconfig_file.full_path().exists() {
                sources.push(Kconfig {
                    file: file.to_string(),
                    ..Default::default()
                });
                continue;
            }
            let source = parse_source_kconfig(input.clone(), source_kconfig_file)?;
            sources.push(source);
        }

        Ok((input, OSource { kconfigs: sources }))
    } else {
        Ok((
            input,
            OSource {
                kconfigs: vec![Kconfig {
                    file: file.to_string(),
                    ..Default::default()
                }],
            },
        ))
    }
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

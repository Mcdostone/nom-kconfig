use std::path::PathBuf;

use crate::{
    entry::{parse_source, Source},
    Kconfig, KconfigFile, KconfigInput,
};

#[test]
fn test_parse_source() {
    assert_parsing_source_eq(
        r#"source "empty""#,
        Ok((
            "",
            Source {
                kconfigs: vec![Kconfig {
                    file: "empty".to_string(),
                    ..Default::default()
                }],
            },
        )),
    )
}

// v2.6.24/arch/cris/arch/drivers/Kconfig
#[test]
fn test_parse_source_no_quote() {
    assert_parsing_source_eq(
        "source empty",
        Ok((
            "",
            Source {
                kconfigs: vec![Kconfig {
                    file: "empty".to_string(),
                    ..Default::default()
                }],
            },
        )),
    )
}

#[test]
#[cfg(not(feature = "kconfiglib"))]
fn test_parse_source_fail_file_not_exist() {
    let res = parse_source(KconfigInput::new_extra(
        "source a/random/file",
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

#[test]
fn test_parse_source_fail_to_parse() {
    let res = parse_source(KconfigInput::new_extra(
        "source \"Cargo.toml\"",
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

#[cfg(not(feature = "coreboot"))]
#[test]
fn test_parse_source_glob_not_supported_without_feature() {
    let res = parse_source(KconfigInput::new_extra(
        "source glob-fixtures/source-child-*.Kconfig",
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

#[cfg(feature = "coreboot")]
#[test]
fn test_parse_source_glob_no_match_fails_with_feature() {
    let res = parse_source(KconfigInput::new_extra(
        "source glob-fixtures/does-not-exist-*.Kconfig",
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ));
    assert!(res.is_ok())
}

pub fn assert_parsing_source_eq(
    input: &str,
    expected: Result<(&str, Source), nom::Err<nom::error::Error<KconfigInput>>>,
) {
    let res = parse_source(KconfigInput::new_extra(
        input,
        KconfigFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ))
    .map(|r| (r.0.fragment().to_owned(), r.1));
    assert_eq!(res, expected)
}

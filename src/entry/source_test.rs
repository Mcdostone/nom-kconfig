use std::{collections::HashMap, path::PathBuf};

use crate::{
    entry::{parse_source, source::apply_vars, Source},
    Kconfig, KconfigFile, KconfigInput,
};

#[test]
fn test_parse_source() {
    assert_parsing_source_eq(
        r#"source "empty""#,
        Ok((
            "",
            Source {
                file: "empty".to_string(),
                entries: vec![],
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
                file: "empty".to_string(),
                entries: vec![],
            },
        )),
    )
}

#[test]
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

fn assert_parsing_source_eq(
    input: &str,
    expected: Result<(&str, Kconfig), nom::Err<nom::error::Error<KconfigInput>>>,
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

fn assert_apply_env_vars(s: &str, extra_vars: &[(&str, &str)], expected: Option<&str>) {
    let extra_vars: HashMap<String, String> = extra_vars
        .iter()
        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
        .collect();
    assert_eq!(apply_vars(s, &extra_vars), expected.map(String::from));
}

#[test]
fn test_apply_env_vars() {
    assert_apply_env_vars("123 $(NON_EXISTENT_VAR) 456", &[], None);
    assert_apply_env_vars(
        "123 $(NON_EXISTENT_VAR) 456",
        &[("USELESS_VAR", "789")],
        None,
    );
    assert_apply_env_vars("123", &[], Some("123"));
    assert_apply_env_vars("123", &[("USELESS_VAR", "789")], Some("123"));
    assert_apply_env_vars(
        "123 $(GOOD_VAR) 456",
        &[("GOOD_VAR", "Bingo")],
        Some("123 Bingo 456"),
    );
    assert_apply_env_vars(
        "123 $(GOOD_VAR) 456 $(GOOD_VAR)",
        &[("GOOD_VAR", "Bingo")],
        Some("123 Bingo 456 Bingo"),
    );
}

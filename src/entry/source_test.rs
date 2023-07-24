use std::path::PathBuf;

use nom_locate::LocatedSpan;

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

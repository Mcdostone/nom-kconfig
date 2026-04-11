use std::path::PathBuf;

use nom::IResult;
#[cfg(feature = "coreboot")]
use nom_kconfig::{entry::config::Config, Entry};
use nom_kconfig::{parse_kconfig, Kconfig, KconfigFile, KconfigInput};

#[cfg(feature = "coreboot")]
#[test]
fn test_source_glob_expands_and_parses_children() {
    let (_content, result) = parse_test_file("source-main.Kconfig");
    let result = result.unwrap();
    let source = source_entry(&result.1);

    assert_eq!(source.entries.len(), 2);
    assert_eq!(
        source.entries[0].file,
        "glob-fixtures/source-child-a.Kconfig"
    );
    assert_eq!(
        source.entries[1].file,
        "glob-fixtures/source-child-b.Kconfig"
    );

    let symbols: Vec<&str> = source
        .entries
        .iter()
        .flat_map(|s| &s.entries)
        .filter_map(config_symbol)
        .collect();

    assert!(symbols.contains(&"GLOB_CHILD_A"));
    assert!(symbols.contains(&"GLOB_CHILD_B"));
}

#[test]
fn test_source_glob_no_match_is_error() {
    let (_content, result) = parse_test_file("source-main-no-match.Kconfig");
    assert!(result.is_err());
}

#[cfg(feature = "coreboot")]
#[test]
fn test_source_literal_path_works_with_glob_feature() {
    let (_content, result) = parse_test_file("source-main-literal.Kconfig");
    let result = result.unwrap();
    let source = source_entry(&result.1);

    assert_eq!(source.entries.len(), 1);
    assert_eq!(
        source.entries[0].file,
        "glob-fixtures/source-child-a.Kconfig"
    );
    assert_eq!(
        config_symbol(&source.entries[0].entries[0]),
        Some("GLOB_CHILD_A")
    );
}

fn parse_test_file(file_name: &str) -> (String, IResult<KconfigInput<'_>, Kconfig>) {
    let tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let input_file = tests_dir.join("glob-fixtures").join(file_name);
    let kconfig_file: KconfigFile = KconfigFile::new(tests_dir, input_file.clone());

    let content = Box::leak(kconfig_file.read_to_string().unwrap().into_boxed_str());
    (
        content.to_string(),
        parse_kconfig(KconfigInput::new_extra(content, kconfig_file)),
    )
}

#[cfg(feature = "coreboot")]
fn source_entry(result: &nom_kconfig::Kconfig) -> &nom_kconfig::entry::Source {
    match result.entries.first() {
        Some(Entry::Source(source)) => source,
        _ => panic!("expected a source entry"),
    }
}

#[cfg(feature = "coreboot")]
fn config_symbol(entry: &Entry) -> Option<&str> {
    match entry {
        Entry::Config(Config { symbol, .. }) => Some(symbol.as_str()),
        _ => None,
    }
}

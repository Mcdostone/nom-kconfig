use crate::{
    assert_parsing_source_eq,
    entry::source::{parse_source, Source},
    KconfigFile,
};

#[test]
fn test_parse_source() {
    let input = "source \"/path/Kconfig\"";
    assert_parsing_source_eq!(
        parse_source,
        input,
        false,
        Ok((
            "",
            Source {
                file: "/path/Kconfig".to_string(),
                entries: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_source_no_quote() {
    let input = "source /path/Kconfig";
    assert_parsing_source_eq!(
        parse_source,
        input,
        false,
        Ok((
            "",
            Source {
                file: "/path/Kconfig".to_string(),
                entries: vec!()
            }
        ))
    )
}

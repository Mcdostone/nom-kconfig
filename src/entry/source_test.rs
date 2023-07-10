use nom::{
    error::{self, ErrorKind},
    Err::Error,
};

use crate::{assert_parsing_eq, entry::source::parse_source};

#[test]
fn test_parse_source() {
    let input = "source \"/path/Kconfig\"";
    assert_parsing_eq!(
        parse_source,
        input,
        Err(Error(error::Error {
            input: KconfigInput::new_extra("", Default::default()),
            code: ErrorKind::Fail,
        }))
    )
}

#[test]
fn test_parse_source_no_quote() {
    let input = "source /path/Kconfig";
    assert_parsing_eq!(
        parse_source,
        input,
        Err(Error(error::Error {
            input: KconfigInput::new_extra("", Default::default()),
            code: ErrorKind::Fail,
        }))
    )
}

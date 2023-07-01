use nom::{Err::Error, error::{ErrorKind, self}};
use nom_locate::LocatedSpan;

use crate::{
    entry::source::parse_source, assert_parsing_eq, KconfigFile, KconfigInput,
};

#[test]
fn test_parse_source() {
    let input = "source \"/path/Kconfig\"";
    assert_parsing_eq!(parse_source, input, 
        Err(Error(error::Error {
                    input: KconfigInput::new_extra("", Default::default()),
                    code: ErrorKind::Fail,
                }
            )))
}

use crate::{
    entry::source::parse_source, assert_parsing_eq,
};

#[test]
fn test_parse_source() {
    let input = "source \"/path/Kconfig\"";
        assert_parsing_eq!(parse_source, input,
        Ok(("", Default::default()))
    )
}

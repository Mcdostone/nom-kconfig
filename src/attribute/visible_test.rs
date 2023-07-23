use crate::{
    assert_parsing_eq,
    attribute::{parse_visible, Visible},
};

#[test]
fn test_parse_type() {
    assert_parsing_eq!(parse_visible, " visible", Ok(("", Visible { r#if: None })))
}

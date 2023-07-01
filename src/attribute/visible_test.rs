use crate::{
    assert_parsing_eq,
    attribute::visible::{parse_visible, Visible},
};

#[test]
fn test_parse_type() {
    let input = " visible";
    assert_parsing_eq!(parse_visible, input, Ok(("", Visible { r#if: None })))
}

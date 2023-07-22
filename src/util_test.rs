use nom::combinator::eof;

use crate::util::{ws, wsi};

#[test]
fn test_ws() {
    let input = "";
    assert_eq!(ws(eof::<&str, ()>)(input), Ok(("", "")))
}

#[test]
fn test_wsi_backslash() {
    let input = r#"   \
          "#;
    assert_eq!(wsi(eof::<&str, ()>)(input), Ok(("", "")))
}

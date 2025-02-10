use nom::{combinator::eof, Parser};

use crate::util::{ws, wsi};

#[test]
fn test_ws() {
    assert_eq!(ws(eof::<&str, ()>).parse("# a comment\n"), Ok(("", "")))
}

#[test]
fn test_wsi_backslash() {
    let input = r"   \
          ";
    assert_eq!(wsi(eof::<&str, ()>).parse(input), Ok(("", "")))
}

use nom::{combinator::eof, error::ParseError};

use crate::util::ws;


#[test]
fn test_ws() {
    let input = "";
    assert_eq!(
        ws(eof::<&str, ()>)(input),
        Ok((
            "",
            ""
        ))
    )
}

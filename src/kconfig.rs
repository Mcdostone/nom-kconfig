use nom::{combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::{
    entry::{parse_entry, Entry},
    util::ws_comment,
    KconfigInput,
};

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Kconfig {
    pub file: String,
    pub entries: Vec<Entry>,
}

pub fn parse_kconfig(input: KconfigInput) -> IResult<KconfigInput, Kconfig> {
    let file = input.extra.file.clone();
    let ok = map(delimited(ws_comment, many0(parse_entry), ws_comment), |d| {
        Kconfig {
            file: file.display().to_string(),
            entries: d,
        }
    })(input);
    // TODO
    #[allow(clippy::let_and_return)]
    ok
}

use nom::{
    combinator::{eof, map},
    multi::many0,
    sequence::delimited,
    IResult,
};
use serde::Serialize;

use crate::{
    entry::{parse_entry, Entry},
    util::{ws, ws_comment},
    KconfigInput,
};

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Kconfig {
    pub file: String,
    pub entries: Vec<Entry>,
}

pub fn parse_kconfig(input: KconfigInput) -> IResult<KconfigInput, Kconfig> {
    let file = input.extra.file.clone();
    let (input, result) = map(delimited(ws_comment, many0(parse_entry), ws_comment), |d| {
        Kconfig {
            file: file.display().to_string(),
            entries: d,
        }
    })(input)?;
    
    let (input, _) = ws(eof)(input)?;
    println!("{:?} {:?}", input.extra.file, input.fragment());
    // TODO
    Ok((input, result))
}

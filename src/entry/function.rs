use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, not_line_ending, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{util::ws, KconfigInput};

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Function {
    pub name: String,
    pub body: String,
}

pub fn parse_function(input: KconfigInput) -> IResult<KconfigInput, Function> {
    map(
        (
            recognize(ws(many1(alt((
                alphanumeric1::<KconfigInput, _>,
                recognize(one_of("_$()")),
            ))))),
            ws(tag("=")),
            ws(terminated(not_line_ending, line_ending)),
        ),
        |(l, _, o)| Function {
            name: l.trim().to_string(),
            body: o.to_string(),
        },
    )
    .parse(input)
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, not_line_ending, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{util::ws, KconfigInput};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Function {
    pub name: String,
    pub body: String,
}

pub fn parse_function(input: KconfigInput) -> IResult<KconfigInput, Function> {
    map(
        tuple((
            recognize(ws(many1(alt((
                alphanumeric1::<KconfigInput, _>,
                recognize(one_of("_$()")),
            ))))),
            ws(tag("=")),
            ws(terminated(not_line_ending, line_ending)),
        )),
        |(l, _, o)| Function {
            name: l.trim().to_string(),
            body: o.to_string(),
        },
    )(input)
}

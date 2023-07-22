use nom::{bytes::complete::tag, combinator::map, multi::many0, sequence::tuple, IResult};
use serde::Serialize;

use crate::{
    attribute::{depends_on::parse_depends_on, prompt::parse_prompt_option, Attribute},
    util::ws,
    KconfigInput,
};

/// This defines a comment which is displayed to the user during the configuration process and is also echoed to the output files. The only possible options are dependencies.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Comment {
    pub prompt: String,
    pub dependencies: Vec<Attribute>,
}

pub fn parse_comment(input: KconfigInput) -> IResult<KconfigInput, Comment> {
    map(
        tuple((
            ws(tag("comment")),
            ws(parse_prompt_option),
            many0(ws(parse_depends_on)),
        )),
        |(_, prompt, dependencies)| Comment {
            prompt: prompt.to_string(),
            dependencies,
        },
    )(input)
}

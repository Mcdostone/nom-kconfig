use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use crate::{util::ws, KconfigInput};

use super::{expression::parse_expression, Attribute};

pub fn parse_depends_on(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    map(
        tuple((tag("depends"), ws(tag("on")), ws(parse_expression))),
        |(_, _, e)| Attribute::DependsOn(e),
    )(input)
}

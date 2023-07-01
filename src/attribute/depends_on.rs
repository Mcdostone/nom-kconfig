use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};

use crate::{util::ws, KconfigInput};

use super::{expression::parse_expression, Attribute};

pub fn parse_depends_on(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    map(pair(tag("depends on"), ws(parse_expression)), |(_, e)| {
        Attribute::DependsOn(e)
    })(input)
}

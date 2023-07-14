use nom::{bytes::complete::tag, combinator::{map, opt}, sequence::tuple, IResult};

use crate::{util::ws, KconfigInput};

use super::{expression::parse_expression, Attribute};


// 2.6.0/arch/v850/Kconfig
pub fn parse_depends_on(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    map(
        tuple((tag("depends"), ws(opt(tag("on"))), ws(parse_expression))),
        |(_, _, e)| Attribute::DependsOn(e),
    )(input)
}
use nom::{bytes::complete::tag, combinator::value, IResult};

use crate::{util::ws, KconfigInput};

pub fn parse_modules(input: KconfigInput) -> IResult<KconfigInput, ()> {
    value((), ws(tag("modules")))(input)
}

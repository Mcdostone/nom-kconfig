use crate::KconfigInput;
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;

pub fn parse_number(input: KconfigInput) -> IResult<KconfigInput, i64> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |d: KconfigInput| FromStr::from_str(d.fragment()),
    )
    .parse(input)
}

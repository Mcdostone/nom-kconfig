use std::ops::{Range, RangeFrom, RangeTo};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, not_line_ending},
    combinator::value,
    error::ParseError,
    multi::many0,
    sequence::{preceded, terminated},
    AsChar, Compare, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Slice,
};

pub fn ws_comment<I, E: ParseError<I>>(input: I) -> IResult<I, (), E>
where
    I: Clone + InputLength + InputTake,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    <I as InputIter>::Item: Clone,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength,
    I: Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
    <I as InputIter>::Item: AsChar,
{
    value(
        (),
        many0(alt((
            preceded(tag("#"), terminated(not_line_ending, line_ending)),
            multispace1,
        ))),
    )(input)
}

pub fn ws<I, F, O, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone + InputLength + InputTake,
    <I as InputIter>::Item: Clone,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength,
    I: Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
    <I as InputIter>::Item: AsChar,
    F: FnMut(I) -> IResult<I, O, E>,
{
    preceded(ws_comment, inner)
}

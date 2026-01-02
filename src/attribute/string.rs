use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{Error, ErrorKind, ParseError},
    sequence::delimited,
    IResult, Input, Parser,
};

use crate::KconfigInput;

pub fn parse_string(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        delimited(tag("\""), take_until_unbalanced('"'), tag("\"")),
        |d| d.fragment().to_string(),
    )
    .parse(input)
}

pub fn take_until_unbalanced(
    delimiter: char,
) -> impl Fn(KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    move |i: KconfigInput| {
        let mut index: usize = 0;
        let mut delimiter_counter = 0;

        let end_of_line = match &i.find('\n') {
            Some(e) => *e,
            None => i.len(),
        };

        while let Some(n) = &i[index..end_of_line].find(delimiter) {
            delimiter_counter += 1;
            index += n + 1;
        }

        // we split just before the last double quote
        index -= 1;
        // Last delimiter is the string delimiter
        delimiter_counter -= 1;

        match delimiter_counter % 2 == 0 {
            true => Ok(i.take_split(index)),
            false => Err(nom::Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            ))),
        }
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    sequence::delimited,
    IResult, Input, Parser,
};

use crate::{util::ws, KconfigInput};

pub fn parse_string(input: KconfigInput) -> IResult<KconfigInput, String> {
    map(
        alt((
            delimited(tag("'"), take_until_unbalanced('\''), tag("'")),
            delimited(tag("\""), take_until_unbalanced('"'), tag("\"")),
        )),
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
        match index.checked_sub(1) {
            Some(i) => index = i,
            None => {
                return Err(nom::Err::Error(Error::from_error_kind(
                    i,
                    ErrorKind::TakeUntil,
                )))
            }
        }
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

/// A first word is `'something here'` or `"something here"` or just a normal word without spaces. It is used in places where Kconfig allows either a string or a symbol, such as in `default` attributes.
pub fn parse_first_word(input: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    alt((
        recognize((tag("'"), take_until_unbalanced('\''), tag("'"))),
        recognize((tag("\""), take_until_unbalanced('"'), tag("\""))),
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("-._'\""))))))),
    ))
    .parse(input)
}

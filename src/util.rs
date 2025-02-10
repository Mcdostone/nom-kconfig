use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, not_line_ending, space1},
    combinator::{eof, value},
    error::ParseError,
    multi::many0,
    sequence::{preceded, terminated},
    AsChar, Compare, IResult, Input, Parser,
};

use crate::KconfigInput;

/// ignores comments
///
/// # Example
/// ```
/// use nom::combinator::eof;
/// use nom::bytes::complete::tag;
/// use nom_kconfig::util::ws_comment;
/// let input = r#"# a comment#   \
///
/// hello"#;
/// assert_eq!(ws_comment::<&str, ()>(input), Ok(("hello", ())))
/// ```
pub fn ws_comment<I, E: ParseError<I>>(input: I) -> IResult<I, (), E>
where
    I: Clone + Input,
    I: Compare<&'static str>,
    <I as Input>::Item: AsChar,
{
    value(
        (),
        many0(alt((
            // TODO 3.0.19/drivers/staging/iio/light/Kconfig, backslash??
            preceded(
                alt((tag("#"), tag("\\#"))),
                terminated(not_line_ending, alt((line_ending, eof))),
            ),
            multispace1,
            // TODO linux v3.2, in file /drivers/dma/Kconfig
            tag(" "),
        ))),
    )
    .parse(input)
}
/// Gets rid of comments, spaces, tabs and newlines.
///
/// # Example
/// ```
/// use nom::bytes::complete::tag;
/// use nom_kconfig::util::ws;
/// let input = r#"# a comment#   \
///
/// hello"#;
/// assert_eq!(ws(tag::<&str, &str, ()>("hello"))(input), Ok(("", "hello")))
/// ```
pub fn ws<I, F, O, E: ParseError<I>>(inner: F) -> impl Parser<I, Output = O, Error = E>
where
    I: Clone + Input,
    I: Compare<&'static str>,
    <I as Input>::Item: AsChar,
    F: Parser<I, Output = O, Error = E>,
{
    preceded(ws_comment, inner)
}

/// Parses the content until EOF or new line.
///
/// # Example
/// ```rust
/// use nom::combinator::map;
/// use nom_kconfig::util::parse_until_eol;
/// use nom_kconfig::KconfigInput;
///
/// let input  = KconfigInput::new_extra("parse me if you\ncan!", Default::default());
/// let (remaining, line) = parse_until_eol(input).unwrap();
/// assert_eq!(line.to_string(), "parse me if you");
/// assert_eq!(remaining.to_string(), "can!");
/// ```
///
pub fn parse_until_eol(input: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    terminated(not_line_ending, alt((line_ending, eof))).parse(input)
}

/// Gets rid of spaces, tabs and backslash + newline.
/// # Example
/// ```
/// use nom::bytes::complete::tag;
/// use nom_kconfig::util::wsi;
/// let input = r#"   \
/// hello"#;
/// assert_eq!(wsi(tag::<&str, &str, ()>("hello"))(input), Ok(("", "hello")))
/// ```
pub fn wsi<I, F, O, E: ParseError<I>>(inner: F) -> impl Parser<I, Output = O, Error = E>
where
    I: Clone + Input,
    I: Compare<&'static str>,
    <I as Input>::Item: AsChar,
    F: Parser<I, Output = O, Error = E>,
{
    preceded(
        value((), many0(alt((preceded(tag("\\"), line_ending), space1)))),
        inner,
    )
}

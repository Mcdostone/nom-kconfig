use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, newline, not_line_ending, space1, tab},
    combinator::{eof, map, opt, peek},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

use crate::{util::ws, KconfigInput};

fn indentation_level(input: KconfigInput) -> IResult<KconfigInput, (usize, usize, usize)> {
    let (input, _) = many0(newline)(input)?;
    // TODO: something is wrong here with the indentation level calculation
    // println!("{:?}", input.chars().next().unwrap());
    map(peek(pair(many0(tab), many0(space1))), |(t, s)| {
        (t.len(), s.len(), s.len() + t.len())
    })(input)
}

pub fn weirdo_help(input: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    // TODO linux v-3.2, in file /drivers/net/ethernet/stmicro/stmmac/Kconfig
    // TODO 3.4.110/drivers/net/ethernet/sfc/Kconfig
    map(
        delimited(
            ws(opt(many0(tag("-")))),
            ws(tag("help")),
            opt(many0(alt((tag("-"), space1)))),
        ),
        |d| d,
    )(input)
}

/// This parses a help text. The end of the help text is determined by the indentation level, this means it ends at the first line which has a smaller indentation than the first line of the help text.
///
/// # Example
/// ```
/// use nom_kconfig::{
///     assert_parsing_eq,
///     attribute::parse_help,
/// };
///
/// assert_parsing_eq!(parse_help, "help\n   hello world", Ok(("", "hello world".to_string())))
/// ```
pub fn parse_help(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (mut input, _) = pair(
        alt((
            ws(tag("help")),
            // TODO linux v-3.2, in file /drivers/net/ethernet/stmicro/stmmac/Kconfig
            weirdo_help,
        )),
        preceded(many0(space1), newline),
    )(input)?;
    //let (input, (tabs, spaces)) = indentation_level(input);
    let r = indentation_level(input);
    let indent: usize;
    match r {
        Ok((i, (_, _, tt))) => {
            input = i;
            indent = tt;
        }
        Err(e) => {
            return match e {
                nom::Err::Error(i) => Ok((i.input, "".to_string())),
                nom::Err::Failure(i) => Ok((i.input, "".to_string())),
                _ => return Err(e),
            };
        }
    }
    if indent == 0 {
        return Ok((input, "".to_string()));
    }
    // TODO see function indentation_level
    //let indent = count(space1::<KconfigInput, _>, indent);
    let indent = space1;

    map(
        many1(alt((
            map(newline, |_| ""),
            map(pair(indent, parse_line_help), |(_, line)| {
                line.fragment().to_owned()
            }),
        ))),
        |v| {
            v.into_iter()
                .map(|l| l.trim_end())
                .filter(|e| !e.is_empty())
                .collect::<Vec<&str>>()
                .join("\n")
        },
    )(input)
}

pub fn parse_line_help(input: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    terminated(not_line_ending, alt((line_ending, eof)))(input)
}

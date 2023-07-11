use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, newline, not_line_ending, space1, tab},
    combinator::{eof, map, opt, peek},
    multi::{count, many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

use crate::{util::ws, KconfigInput};

fn indentation_level(input: KconfigInput) -> IResult<KconfigInput, (usize, usize)> {
    let (input, _) = many0(newline)(input)?;
    // TODO: something is wrong here with the indentation level calculation
    // println!("{:?}", input.chars().next().unwrap());
    map(peek(pair(many0(tab), many0(space1))), |(t, s)| {
        (t.len(), s.len())
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

pub fn parse_help(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (mut input, _) = pair(
        alt((
            ws(tag("help")),
            // TODO linux v-3.2, in file /drivers/net/ethernet/stmicro/stmmac/Kconfig
            weirdo_help, //preceded(ws(tag("--")), ws(tag("help"))),
                         // TODO linux v-3.2, in file /net/caif/Kconfig
                         //delimited(tag("---"), ws(tag("help")), ws(tag("---"))), // unit test test_parse_help_space
        )),
        preceded(many0(space1), newline),
    )(input)?;
    //let (input, (tabs, spaces)) = indentation_level(input);
    let r = indentation_level(input);
    let spaces: usize;
    match r {
        Ok((i, (_, s))) => {
            input = i;
            spaces = s;
        }
        Err(e) => {
            return match e {
                nom::Err::Error(i) => Ok((i.input, "".to_string())),
                nom::Err::Failure(i) => Ok((i.input, "".to_string())),
                _ => return Err(e),
            };
        }
    }
    if spaces == 0 {
        return Ok((input, "".to_string()));
    }
    // TODO see function indentation_level
    //println!("{} {}", _tabs, spaces);
    let indent = count(space1::<KconfigInput, _>, spaces);
    let content_line = terminated(not_line_ending, alt((line_ending, eof)));
    map(
        many1(alt((
            map(newline, |_| ""),
            map(pair(indent, content_line), |(_, line)| {
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

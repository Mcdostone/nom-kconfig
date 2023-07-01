use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, newline, not_line_ending, space1, tab},
    combinator::{eof, map, peek},
    multi::{count, many0, many1},
    sequence::{pair, terminated},
    IResult,
};

use crate::KconfigInput;

fn indentation_level(input: KconfigInput) -> IResult<KconfigInput, (usize, usize)> {
    let (input, _) = many0(newline)(input)?;
    // TODO: something is wrong here with the indentation level calculation
    // println!("{:?}", input.chars().next().unwrap());
    let (input, (tabs, spaces)) = peek(pair(many0(tab), many0(space1)))(input)?;
    Ok(((input), (tabs.len(), spaces.len())))
}

pub fn parse_help(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (input, _) = pair(tag("help"), newline)(input)?;
    let (input, (_tabs, spaces)) = indentation_level(input)?;
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

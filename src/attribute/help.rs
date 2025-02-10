use nom::bytes::complete::take_while;
use nom::multi::fold_many0;
use nom::Parser;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, newline, not_line_ending, space1},
    combinator::{eof, map, opt, peek},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::{util::ws, KconfigInput};

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
    )
    .parse(input)
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
    // parse out help tag
    let (input, _) = pair(
        alt((
            ws(tag("help")),
            // TODO linux v-3.2, in file /drivers/net/ethernet/stmicro/stmmac/Kconfig
            weirdo_help,
        )),
        preceded(many0(space1), newline),
    )
    .parse(input)?;

    // parse the raw text
    let (input, text) = parse_help_text(input)?;
    Ok((input, text))
}

fn parse_help_text(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (original, initial_indentation_len) = peek_indentation(input)?;
    if initial_indentation_len == 0 {
        return Ok((original, String::new()));
    }

    // Remove initial indentation and parse first line
    let (input, _) = take(initial_indentation_len)(original)?;
    let (remaining, first_line) = parse_full_help_line(input)?;

    // Parse subsequent lines while maintaining indentation
    let (remaining, mut help_text) = fold_many0(
        |i| {
            let (orig, indent_len) = peek_indentation(i)?;

            if (indent_len != 0) && (indent_len < initial_indentation_len) {
                return Err(nom::Err::Error(nom::error::Error::new(
                    orig,
                    nom::error::ErrorKind::Fail, // Stop parsing when indentation decreases
                )));
            } else if indent_len == 0 {
                // handle newlines between paragraph's
                return parse_newline_only(orig);
            } else {
                // Consume the same base indentation.
                let (remain, _) = take(initial_indentation_len)(orig)?;
                // Parse the raw help line text.
                return parse_full_help_line(remain);
            }
        },
        String::new,
        |mut acc, line| {
            acc.push_str(&line);
            acc
        },
    )
    .parse(remaining)?;

    help_text.insert_str(0, &first_line);

    Ok((remaining, help_text.trim().to_string()))
}

fn parse_line_help(input: KconfigInput) -> IResult<KconfigInput, (KconfigInput, KconfigInput)> {
    pair(not_line_ending, alt((line_ending, eof))).parse(input)
}

fn parse_full_help_line(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (input, (raw_text, line_end)) = parse_line_help(input)?;
    let mut parsed_line = raw_text.to_string();
    parsed_line.push_str(line_end.fragment());
    Ok((input, parsed_line))
}

fn parse_newline_only(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (input, newline) = newline(input)?;
    Ok((input, newline.to_string()))
}
fn peek_til_newline(s: KconfigInput) -> IResult<KconfigInput, (KconfigInput, KconfigInput)> {
    peek(parse_line_help).parse(s)
}

fn peek_indentation(s: KconfigInput) -> IResult<KconfigInput, usize> {
    let (original, peeked) = peek_til_newline(s)?;
    let (_, len) = indentation_level(peeked.0)?;
    Ok((original, len))
}

fn indentation_level(input: KconfigInput) -> IResult<KconfigInput, usize> {
    // Assumes that tab and space usage is consistent across multi line text.
    // E.g We don't know how much spaces a tab is.
    //
    // Example:
    // help\n
    //     Lorem Ipsum\n
    // \t\tLorem Ipsum\n
    //
    // The above would consider the second text line to have less indentation( 4 white spaces vs 2 tabs as we only count chars) and thous only include the first line.
    let (input, indent) = take_while(|c: char| c == ' ' || c == '\t')(input)?;
    let len = indent.fragment().len();

    Ok((input, (len)))
}

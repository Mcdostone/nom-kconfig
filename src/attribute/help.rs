use crate::{util::ws, KconfigInput};
use nom::bytes::complete::take_while;
use nom::character::complete::space0;
use nom::combinator::recognize;
use nom::multi::fold_many0;
use nom::sequence::terminated;
use nom::Parser;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, newline, not_line_ending, space1},
    combinator::{eof, map, opt, peek},
    multi::many0,
    sequence::delimited,
    IResult,
};

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
    let (input, _) = (
        alt((
            ws(tag("help")),
            // TODO linux v-3.2, in file /drivers/net/ethernet/stmicro/stmmac/Kconfig
            weirdo_help,
        )),
        many0(space1),
        newline,
    )
        .parse(input)?;

    // parse the raw text
    let (input, text) = parse_help_text(input)?;
    Ok((input, text))
}

//fn parse_help_text_2(input: KconfigInput) -> IResult<KconfigInput, String> {
//    let (original, initial_indentation_len) = peek_initial_indentation(input)?;
//    if initial_indentation_len.chars == 0 {
//        return Ok((original, String::new()));
//    }
//
//    // parse the help text
//    // if the line contains only newline, we should accept it as well
//    // if the indentation is the same or larger, we should continue parsing, whitespaces should not be trimmed when the current indentation is larger than the initial one
//    // if the indentation decreases, we should stop parsing
//    // newlines counts
//
//    let mut remaining = original;
//    let mut help_text = String::new();
//    loop {
//        let (next_input, current_indentation_len) = peek_indentation(remaining)?;
//        let peek_line = peek_til_newline(next_input)?.1;
//        if current_indentation_len < initial_indentation_len && peek_line.fragment().trim() != "" {
//            break;
//        }
//
//
//        // parse either a full help line or a newline only
//        let parse_result = alt((parse_full_help_line, parse_newline_only)).parse(remaining);
//        match parse_result {
//            Ok((next_input, parsed_line)) => {
//                help_text.push_str(&parsed_line);
//                remaining = next_input;
//            }
//            Err(_) => {
//                break;
//            }
//        }
//    }
//
//    Ok((remaining, help_text.trim().to_string()))
//}

fn parse_help_text(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (original, initial_indentation_len) = peek_initial_indentation(input)?;
    if initial_indentation_len.chars == 0 {
        return Ok((original, String::new()));
    }

    // Parse subsequent lines while maintaining indentation
    let (remaining, help_text) = fold_many0(
        |i| {
            let (orig, indent_len) = peek_indentation(i)?;
            let peek_line = peek_til_newline(orig)?;

            if peek_line.1.fragment().trim() == "" {
                // allow empty lines
                parse_newline_only(peek_line.0)
            } else if indent_len < initial_indentation_len {
                // Stop parsing when indentation decreases
                Err(nom::Err::Error(nom::error::Error::new(
                    peek_line.1,
                    nom::error::ErrorKind::Fail,
                )))
            } else {
                // Consume the same base indentation.
                let (remain, _) =
                    take(indent_len.chars.min(initial_indentation_len.chars))(peek_line.0)?;
                // Parse the raw help line text.
                parse_full_help_line(remain)
            }
        },
        //            if (indent_len.chars != 0) && (indent_len < initial_indentation_len) {
        //                return Err(nom::Err::Error(nom::error::Error::new(
        //                    orig,
        //                    nom::error::ErrorKind::Fail, // Stop parsing when indentation decreases
        //                )));
        //            } else if indent_len.chars == 0 {
        //                // handle newlines between paragraph's
        //                parse_newline_only(orig)
        //            } else {
        //                // Consume the same base indentation.
        //                let (remain, _) = take(initial_indentation_len.chars.min(indent_len.chars))(orig)?;
        //                // Parse the raw help line text.
        //                parse_full_help_line(remain)
        //            }
        //        },
        String::new,
        |mut acc, line| {
            acc.push_str(&line);
            acc
        },
    )
    .parse(original)?;

    Ok((remaining, help_text.trim_end().to_string()))
}

fn parse_line_help(input: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    //pair(not_line_ending, alt((line_ending, eof))).parse(input)
    terminated(not_line_ending, opt(alt((line_ending, eof)))).parse(input)
}

fn parse_full_help_line(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (input, raw_text) = parse_line_help(input)?;
    let mut parsed_line = raw_text.to_string();
    parsed_line.push('\n');
    Ok((input, parsed_line))
}

fn parse_newline_only(input: KconfigInput) -> IResult<KconfigInput, String> {
    let (input, newline) = newline(input)?;
    Ok((input, newline.to_string()))
}
fn peek_til_newline(s: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    peek(parse_line_help).parse(s)
}

// Parser that consumes empty lines (lines with only whitespace or nothing)
fn empty_line(s: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    recognize(terminated(space0, line_ending)).parse(s)
}

// Peek at the first non-empty line without consuming it
fn peek_first_non_empty_line(s: KconfigInput) -> IResult<KconfigInput, KconfigInput> {
    let (original, (_, non_empty_line)) = peek((many0(empty_line), not_line_ending)).parse(s)?;
    Ok((original, non_empty_line))
    //let (input, first_none_empty_line) = peek(many0(empty_line)).parse(s)?;
    //peek(not_line_ending).parse(input)
}

/// find the indentation level
fn peek_initial_indentation(s: KconfigInput) -> IResult<KconfigInput, IndentationLevel> {
    let (original, peeked) = peek_first_non_empty_line.parse(s)?;
    let (_, len) = indentation_level(peeked)?;
    Ok((original, len))
}

fn peek_indentation(s: KconfigInput) -> IResult<KconfigInput, IndentationLevel> {
    let (original, peeked) = peek_til_newline(s)?;
    let (_, len) = indentation_level(peeked)?;
    Ok((original, len))
}

/// Inspired from https://github.com/movidius/kconfig-frontends/blob/44b2a3287ebd5be5b49e51feaafb9c54c9f0fe41/libs/parser/lconf.l#L204-L250
fn indentation_level(input: KconfigInput) -> IResult<KconfigInput, IndentationLevel> {
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

    let mut computed = 0;
    for c in indent.fragment().chars() {
        match c {
            '\t' => {
                computed = (computed & !7) + 8;
            }
            ' ' => computed += 1,
            _ => unreachable!(
                "This should never happen because indentation only takes spaces and tabs"
            ),
        }
    }

    Ok((
        input,
        IndentationLevel {
            chars: indent.fragment().len(),
            computed,
        },
    ))
}

#[derive(PartialEq, Debug)]
struct IndentationLevel {
    chars: usize,
    computed: usize,
}

// need to use the operator < on IndentationLevel
impl PartialOrd for IndentationLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.computed.partial_cmp(&other.computed)
    }
}

#[cfg(test)]
use crate::assert_parsing_eq;
#[test]
fn test_peek_initial_indentation_first_empty_line() {
    let input = r#"
	  This is a general notification"#;

    assert_parsing_eq!(
        peek_initial_indentation,
        input,
        Ok((
            "\n\t  This is a general notification",
            IndentationLevel {
                chars: 3,
                computed: 10
            }
        ))
    )
}

#[test]
fn test_peek_initial_indentation() {
    let input = r#"    first word"#;
    assert_parsing_eq!(
        peek_initial_indentation,
        input,
        Ok((
            "    first word",
            IndentationLevel {
                chars: 4,
                computed: 4
            }
        ))
    )
}

#[test]
fn test_indentation_level() {
    assert_parsing_eq!(
        indentation_level,
        "\t",
        Ok((
            "",
            IndentationLevel {
                chars: 1,
                computed: 8
            }
        ))
    );
    assert_parsing_eq!(
        indentation_level,
        " \t",
        Ok((
            "",
            IndentationLevel {
                chars: 2,
                computed: 8
            }
        ))
    );
    assert_parsing_eq!(
        indentation_level,
        "  \t",
        Ok((
            "",
            IndentationLevel {
                chars: 3,
                computed: 8
            }
        ))
    );
    assert_parsing_eq!(
        indentation_level,
        "        \t",
        Ok((
            "",
            IndentationLevel {
                chars: 9,
                computed: 16
            }
        ))
    );
}

#[test]
fn test_peek_first_non_empty_line() {
    let input = "        \t\n\n  hello";
    let (remaining, line) = peek_first_non_empty_line
        .parse(KconfigInput::from(input))
        .unwrap();
    assert_eq!(remaining.fragment(), &input);
    assert_eq!(line.fragment(), &"  hello");
}

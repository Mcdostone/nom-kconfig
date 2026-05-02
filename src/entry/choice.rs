use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    sequence::{delimited, pair},
    IResult, Parser,
};

#[cfg(feature = "named-choice")]
use nom::{
    character::complete::{alphanumeric1, one_of},
    combinator::recognize,
    multi::many1,
};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(feature = "named-choice")]
use crate::attribute::string::parse_string;
use crate::{
    attribute::{optional::parse_optional, parse_attribute, r#type::parse_type, Attribute},
    util::ws,
    Entry, KconfigInput,
};

use super::parse_entry;

/// This defines a choice group and accepts any of the above attributes as options. A choice can only be of type bool or tristate. If no type is specified for a choice, its type will be determined by the type of the first choice element in the group or remain unknown if none of the choice elements have a type specified, as well.
///
/// While a boolean choice only allows a single config entry to be selected, a tristate choice also allows any number of config entries to be set to 'm'. This can be used if multiple drivers for a single hardware exists and only a single driver can be compiled/loaded into the kernel, but all drivers can be compiled as modules.
///
/// A choice accepts another option "optional", which allows to set the choice to 'n' and no entry needs to be selected. If no [symbol](crate::symbol::Symbol) is associated with a choice, then you can not have multiple definitions of that choice. If a [symbol](crate::symbol::Symbol) is associated to the choice, then you may define the same choice (i.e. with the same entries) in another place.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Choice {
    /// Only possible in the named_choice Kconfig format
    #[cfg(feature = "named-choice")]
    pub name: Option<String>,
    pub options: Vec<Attribute>,
    pub entries: Vec<Entry>,
}

fn parse_choice_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    ws(many0(alt((
        parse_attribute,
        parse_type,
        map(ws(parse_optional), |_| Attribute::Optional),
    ))))
    .parse(input)
}

fn parse_choice_simple(input: KconfigInput) -> IResult<KconfigInput, Choice> {
    map(
        delimited(
            tag("choice"),
            pair(parse_choice_attributes, many0(ws(parse_entry))),
            ws(tag("endchoice")),
        ),
        |(options, entries)| {
            #[cfg(feature = "named-choice")]
            return Choice {
                options,
                entries,
                name: None,
            };
            #[cfg(not(feature = "named-choice"))]
            return Choice { options, entries };
        },
    )
    .parse(input)
}

pub fn parse_choice(input: KconfigInput) -> IResult<KconfigInput, Choice> {
    alt((
        parse_choice_simple,
        #[cfg(feature = "named-choice")]
        parse_named_choice,
    ))
    .parse(input)
}

#[cfg(feature = "named-choice")]
pub fn parse_named_choice(input: KconfigInput) -> IResult<KconfigInput, Choice> {
    map(
        delimited(
            tag("choice"),
            (
                ws(parse_choice_name),
                parse_choice_attributes,
                many0(ws(parse_entry)),
            ),
            ws(tag("endchoice")),
        ),
        |(name, options, entries)| Choice {
            options,
            entries,
            name: Some(name),
        },
    )
    .parse(input)
}

#[cfg(feature = "named-choice")]
pub fn parse_choice_name(input: KconfigInput) -> IResult<KconfigInput, String> {
    alt((
        map(
            recognize(ws(many1(alt((alphanumeric1, recognize(one_of("._"))))))),
            |c: KconfigInput| c.trim().to_string(),
        ),
        parse_string,
    ))
    .parse(input)
}

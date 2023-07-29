use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    sequence::{pair, terminated},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{optional::parse_optional, parse_attribute, Attribute},
    util::ws,
    KconfigInput,
};

use super::{
    config::{parse_bool_config, parse_tristate_config},
    Config,
};

/// This defines a choice group and accepts any of the above attributes as options. A choice can only be of type bool or tristate. If no type is specified for a choice, its type will be determined by the type of the first choice element in the group or remain unknown if none of the choice elements have a type specified, as well.
///
/// While a boolean choice only allows a single config entry to be selected, a tristate choice also allows any number of config entries to be set to 'm'. This can be used if multiple drivers for a single hardware exists and only a single driver can be compiled/loaded into the kernel, but all drivers can be compiled as modules.
///
/// A choice accepts another option "optional", which allows to set the choice to 'n' and no entry needs to be selected. If no [symbol](crate::symbol::Symbol) is associated with a choice, then you can not have multiple definitions of that choice. If a [symbol](crate::symbol::Symbol) is associated to the choice, then you may define the same choice (i.e. with the same entries) in another place.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Choice {
    pub options: Vec<Attribute>,
    pub configs: Vec<Config>,
}

fn parse_choice_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    ws(many0(alt((
        parse_attribute,
        map(ws(parse_optional), |_| Attribute::Optional),
    ))))(input)
}

pub fn parse_choice(input: KconfigInput) -> IResult<KconfigInput, Choice> {
    let (input, _) = tag("choice")(input)?;
    map(
        terminated(
            pair(
                parse_choice_attributes,
                many0(ws(alt((parse_bool_config, parse_tristate_config)))),
            ),
            ws(tag("endchoice")),
        ),
        |(options, configs)| Choice { options, configs },
    )(input)
}

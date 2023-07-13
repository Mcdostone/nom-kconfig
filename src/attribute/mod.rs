pub mod def_bool;
pub mod def_tristate;
pub mod default;
pub mod depends_on;
pub mod expression;
pub mod function;
pub mod help;
pub mod imply;
pub mod modules;
pub mod option;
pub mod optional;
pub mod prompt;
pub mod requires;
pub mod range;
pub mod select;
pub mod r#type;
pub mod visible;

use nom::{branch::alt, combinator::map, multi::many0, IResult};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use self::{
    def_bool::{parse_def_bool, DefBool},
    def_tristate::{parse_def_tristate, DefTristate},
    default::{parse_default, DefaultAttribute},
    depends_on::parse_depends_on,
    expression::Expression,
    help::parse_help,
    imply::{parse_imply, Imply},
    modules::parse_modules,
    option::{parse_option, OptionValues},
    prompt::{parse_prompt, Prompt},
    r#type::{parse_type, EntryType},
    range::{parse_range, Range},
    select::{parse_select, Select},
    visible::{parse_visible, Visible}, requires::{Requires, parse_requires},
};

pub fn parse_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    ws(many0(parse_attribute))(input)
}

pub fn parse_attribute(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    alt((
        map(ws(parse_def_bool), Attribute::DefBool),
        map(ws(parse_type), Attribute::Type),
        map(ws(parse_prompt), Attribute::Prompt),
        map(ws(parse_help), Attribute::Help),
        ws(parse_depends_on),
        map(ws(parse_select), Attribute::Select),
        map(ws(parse_default), Attribute::Default),
        map(ws(parse_requires), Attribute::Requires),
        map(ws(parse_def_tristate), Attribute::DefTristate),
        map(ws(parse_modules), |_| Attribute::Modules),
        map(ws(parse_range), Attribute::Range),
        map(ws(parse_imply), Attribute::Imply),
        map(ws(parse_visible), Attribute::Visible),
        map(ws(parse_option), Attribute::Option),
    ))(input)
}
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Attribute {
    Help(String),
    Prompt(Prompt),
    Modules,
    Type(EntryType),
    Select(Select),
    DependsOn(Expression),
    Optional,
    Range(Range),
    Visible(Visible),
    Default(DefaultAttribute),
    DefBool(DefBool),
    DefTristate(DefTristate),
    Imply(Imply),
    Requires(Requires),
    Option(OptionValues),
}

#[cfg(test)]
mod def_bool_test;
#[cfg(test)]
mod def_tristate_test;
#[cfg(test)]
mod default_test;
#[cfg(test)]
mod depends_on_test;
#[cfg(test)]
mod expression_test;
#[cfg(test)]
mod function_test;
#[cfg(test)]
mod help_test;
#[cfg(test)]
mod imply_test;
#[cfg(test)]
mod modules_test;
#[cfg(test)]
mod option_test;
#[cfg(test)]
mod optional_test;
#[cfg(test)]
mod prompt_test;
#[cfg(test)]
mod range_test;
#[cfg(test)]
mod select_test;
#[cfg(test)]
mod type_test;
#[cfg(test)]
mod visible_test;
#[cfg(test)]
mod requires_test;

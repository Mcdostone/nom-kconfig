//! Module defining the different attributes.
//! A entry can have a number of attributes: [https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes)

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
pub mod range;
pub mod requires;
pub mod select;
pub mod r#type;
pub mod visible;

use nom::{branch::alt, combinator::map, multi::many0, IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use self::r#type::ConfigType;
pub use self::{
    default::{parse_default, DefaultAttribute},
    depends_on::parse_depends_on,
    expression::Expression,
    help::parse_help,
    imply::{parse_imply, Imply},
    modules::parse_modules,
    option::{parse_option, OptionValues},
    prompt::{parse_prompt, Prompt},
    range::{parse_range, Range},
    requires::parse_requires,
    select::{parse_select, Select},
    visible::parse_visible,
};

pub use self::expression::{
    parse_expression, parse_if_attribute, AndExpression, Atom, CompareExpression, CompareOperator,
    OrExpression, Term,
};
pub use self::function::{parse_function_call, ExpressionToken, FunctionCall, Parameter};
pub use self::optional::parse_optional;
pub use self::prompt::parse_prompt_value;

/// Official documentation regarding the different attributes: [https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html#menu-attributes)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Attribute {
    Help(String),
    Prompt(Prompt),
    Modules,
    Select(Select),
    DependsOn(Expression),
    Optional,
    Range(Range),
    Visible(Option<Expression>),
    Default(DefaultAttribute),
    Imply(Imply),
    Requires(Expression),
    Type(ConfigType),
    Option(OptionValues),
}

pub fn parse_attributes(input: KconfigInput) -> IResult<KconfigInput, Vec<Attribute>> {
    ws(many0(parse_attribute)).parse(input)
}

pub fn parse_attribute(input: KconfigInput) -> IResult<KconfigInput, Attribute> {
    alt((
        map(ws(parse_prompt), Attribute::Prompt),
        map(ws(parse_help), Attribute::Help),
        ws(parse_depends_on),
        map(ws(parse_select), Attribute::Select),
        map(ws(parse_default), Attribute::Default),
        map(ws(parse_requires), Attribute::Requires),
        map(ws(parse_modules), |_| Attribute::Modules),
        map(ws(parse_range), Attribute::Range),
        map(ws(parse_imply), Attribute::Imply),
        map(ws(parse_visible), Attribute::Visible),
        map(ws(parse_option), Attribute::Option),
    ))
    .parse(input)
}

#[cfg(feature = "display")]
use std::fmt::Display;
#[cfg(feature = "display")]
impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Help(s) => write!(f, "help\n  {}", s),
            Attribute::Prompt(p) => write!(f, "prompt {p}"),
            Attribute::Modules => write!(f, "modules"),
            Attribute::Select(s) => write!(f, "select {s}"),
            Attribute::DependsOn(d) => write!(f, "depends on {d}"),
            Attribute::Optional => write!(f, "optional"),
            Attribute::Range(r) => write!(f, "range {r}"),
            Attribute::Visible(v) => match v {
                Some(e) => write!(f, "visible if {e}"),
                None => write!(f, "visible"),
            },
            Attribute::Default(d) => write!(f, "default {d}"),
            Attribute::Imply(i) => write!(f, "imply {i}"),
            Attribute::Requires(r) => write!(f, "requires {r}"),
            Attribute::Type(t) => write!(f, "{t}"),
            Attribute::Option(o) => write!(f, "option {o}"),
        }
    }
}

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
mod mod_test;
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
mod requires_test;
#[cfg(test)]
mod select_test;
#[cfg(test)]
mod type_test;
#[cfg(test)]
mod visible_test;

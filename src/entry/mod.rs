use nom::{branch::alt, combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::{
    attribute::function::{parse_function_call, FunctionCall},
    util::{ws, ws_comment},
    KconfigInput,
};

use self::{
    choice::{parse_choice, Choice},
    comment::{parse_comment, Comment},
    config::{parse_config, Config},
    function::{parse_function, Function},
    main_menu::{parse_main_menu, MainMenu},
    menu::{parse_menu, Menu},
    menuconfig::{parse_menu_config, MenuConfig},
    r#if::{parse_if, If},
    source::{parse_source, Source},
    variable::{parse_variable_assignment, VariableAssignment},
};

pub mod choice;
pub mod comment;
pub mod config;
pub mod function;
pub mod r#if;
pub mod main_menu;
pub mod menu;
pub mod menuconfig;
pub mod source;
pub mod variable;

#[cfg(test)]
pub mod choice_test;
#[cfg(test)]
mod comment_test;
#[cfg(test)]
mod config_test;
#[cfg(test)]
mod function_test;
#[cfg(test)]
pub mod if_test;
#[cfg(test)]
mod main_menu_test;
#[cfg(test)]
mod menu_test;
#[cfg(test)]
mod menuconfig_test;
#[cfg(test)]
mod source_test;
#[cfg(test)]
mod variable_test;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Entry {
    Config(Config),
    Choice(Choice),
    MenuConfig(MenuConfig),
    Menu(Menu),
    Comment(Comment),
    Source(Source),
    VariableAssignment(VariableAssignment),
    FunctionCall(FunctionCall),
    Function(Function),
    If(If),
    MainMenu(MainMenu),
}

pub fn parse_entry(input: KconfigInput) -> IResult<KconfigInput, Entry> {
    alt((
        map(ws(parse_config), Entry::Config),
        map(ws(parse_choice), Entry::Choice),
        map(ws(parse_menu_config), Entry::MenuConfig),
        map(ws(parse_function), Entry::Function),
        map(ws(parse_main_menu), Entry::MainMenu),
        map(ws(parse_if), Entry::If),
        map(ws(parse_menu), Entry::Menu),
        map(ws(parse_comment), Entry::Comment),
        map(ws(parse_source), Entry::Source),
        map(ws(parse_variable_assignment), Entry::VariableAssignment),
        map(ws(parse_function_call), Entry::FunctionCall),
    ))(input)
}

pub fn parse_entries(input: KconfigInput) -> IResult<KconfigInput, Vec<Entry>> {
    delimited(ws_comment, many0(parse_entry), ws_comment)(input)
}

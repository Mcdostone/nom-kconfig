use nom::{branch::alt, combinator::map, IResult};
use serde::Serialize;

use crate::{attribute::function::{FunctionCall, parse_function_call}, util::ws};

use super::{config::{Config, parse_config}, choice::{Choice, parse_choice}, menuconfig::{MenuConfig, parse_menu_config}, menu::{Menu, parse_menu}, comment::{Comment, parse_comment}, source::{Source, parse_source}, r#if::{If, parse_if}, main_menu::{MainMenu, parse_main_menu}, function::{parse_function, Function}, variable::{VariableAssignment, parse_variable_assignment}};


/* 
pub fn parse_entry(input: &str, parser: Parser) -> IResult<&str, Entry> {
    alt((
        map(ws(parse_config), Entry::Config),
        map(ws(parse_choice), Entry::Choice),
        map(ws(parse_menu_config), Entry::MenuConfig),
        map(ws(parse_function), Entry::Function),
        map(ws(parse_main_menu), Entry::MainMenu),
        map(ws(parse_if), Entry::If),
        map(ws(parse_menu), Entry::Menu),
        map(ws(parse_comment), Entry::Comment),
        map(ws(parse_source,), Entry::Source),
        map(ws(parse_variable_assignment), Entry::VariableAssignment),
        map(ws(parse_function_call), Entry::FunctionCall),
    ))(input)
}

*/
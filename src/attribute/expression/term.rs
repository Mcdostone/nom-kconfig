use crate::attribute::expression::parse_atom;
use crate::attribute::Atom;
use crate::util::wsi;
use crate::KconfigInput;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{IResult, Parser};
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "display")]
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Term {
    Not(Atom),
    Atom(Atom),
}

#[cfg(feature = "display")]
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Not(atom) => write!(f, "!{}", atom),
            Term::Atom(atom) => write!(f, "{}", atom),
        }
    }
}

pub fn parse_term(input: KconfigInput) -> IResult<KconfigInput, Term> {
    alt((
        map(preceded(wsi(tag("!")), parse_atom), Term::Not),
        map(parse_atom, Term::Atom),
    ))
    .parse(input)
}

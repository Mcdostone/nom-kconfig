use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Tristate {
    Yes,
    Module,
    No,
}

impl Display for Tristate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tristate::Yes => write!(f, "y"),
            Tristate::Module => write!(f, "m"),
            Tristate::No => write!(f, "n"),
        }
    }
}

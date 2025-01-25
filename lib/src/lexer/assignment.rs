use std::fmt::{write, Debug, Display};

use super::Token;

/// Should this really be a different thing?
#[derive(Clone, Copy, PartialEq)]
pub struct Assignment {}

impl Token for Assignment {
    fn parse(text: &str) -> Option<(Self, usize)> {
        // no regex needed!
        if &text[0..1] == "=" {
            Some((Assignment {}, 1))
        } else { None }
    }
    fn raw(&self) -> &str {
        "="
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assignment (=)")
    }
}

impl Debug for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
//! This module lexes all reserved keywords in Lua

use macros::VariantsToStr;
use static_assertions::const_assert;
use super::*;

// check that the list is sorted
const fn is_sorted(a: &[&str]) -> bool {
    let mut i = 1;
    while i < a.len() {
        if a[i-1].len() < a[i].len() {return false;}
        i += 1;
    }
    true
}

#[derive(VariantsToStr)]
pub enum Keyword {
    Function,
    Elseif,
    Repeat,
    Return,
    Break,
    False,
    Local,
    Until,
    While,
    Else,
    Then,
    True,
    And,
    End,
    For,
    Nil,
    Not,
    Do,
    If,
    In,
    Or,
}

const_assert!(is_sorted(&Keyword::all_variants_str()));

impl Token for Keyword {
    fn parse(text: &str) -> Option<(Self, usize)> {
        Keyword::str_to_variant(text)
    }
    fn raw(&self) -> &str {
        todo!()
    }
}
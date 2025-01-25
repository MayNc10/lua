//! This module supports capturing any operator token
//! Importantly, it does not discriminate between binary and unary operators
//! That will be the job of the parser
//! (Yes, I know I could use a standard EBNF parser, but I want to do it myself)

use super::*;

mod macros {
    macro_rules! capture {
        ($text:ident, $len:expr, $($str:literal, $enum:path),+) => {
            // header
            if $text.len() >= $len {
                match &$text[..$len] {
                    $(
                        $str => Some(($enum, $len)),
                    )+
                    _ => None
                }
            } else { None }
        };
    }
    pub(super) use capture;

    macro_rules! single_capture {
        ($text:ident, $($str:literal, $enum:path),+) => {
            macros::capture! {$text, 1, $($str, $enum),+}
        };
    }
    pub(super) use single_capture;

    macro_rules! double_capture {
        ($text:ident, $($str:literal, $enum:path),+) => { 
            macros::capture! {$text, 2, $($str, $enum),+}
        };
    }
    pub(super) use double_capture;
    
    macro_rules! triple_capture {
        ($text:ident, $($str:literal, $enum:path),+) => {
            macros::capture! {$text, 3, $($str, $enum),+}
        };
    }
    pub(super) use triple_capture;
}

/// Important: to avoid lookahead, parse this before assignment operations
/// e.g. '==' here could appear as '=', so if we parse assignment before, we'll always parse '==' as '=' '=' and fail
// TODO: Rename some of these?
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operator {
    LogicalOr,
    LogicalAnd,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Equal,
    BitOr,
    Tilde, 
    BitAnd,
    LeftShift,
    RightShift,
    Range,
    Plus,
    Minus,
    Star,
    Slash,
    DoubleSlash,
    Percent,
    LogicalNot,
    Hash,
    Caret,
}

impl Token for Operator {
    fn parse(text: &str) -> Option<(Self, usize)> {
        // parse longest first
        macros::triple_capture!{text,
            "and", Operator::LogicalAnd,
            "not", Operator::LogicalNot 
        }.or_else(|| 
        macros::double_capture!{text,
            "or", Operator::LogicalOr,
            "<=", Operator::LessEqual,
            ">=", Operator::GreaterEqual, 
            "~=", Operator::NotEqual,
            "==", Operator::Equal,
            "<<", Operator::LeftShift,
            ">>", Operator::RightShift,
            "..", Operator::Range,
            "//", Operator::DoubleSlash
        }).or_else(|| 
        macros::single_capture!{text,
            "<", Operator::Less,
            ">", Operator::Greater,
            "|", Operator::BitOr,
            "&", Operator::BitAnd,
            "+", Operator::Plus,
            "-", Operator::Minus,
            "*", Operator::Star,
            "/", Operator::Slash,
            "%", Operator::Percent,
            "#", Operator::Hash,
            "~", Operator::Tilde,
            "^", Operator::Caret
        })
    }
    fn raw(&self) -> &str {
        match self {
            Operator::LogicalAnd => "and",
            Operator::LogicalNot => "not",
            Operator::LogicalOr => "or",
            Operator::LessEqual => "<=",
            Operator::GreaterEqual => ">=",
            Operator::NotEqual => "~=",
            Operator::Equal => "==",
            Operator::LeftShift => "<<",
            Operator::RightShift => ">>",
            Operator::Range => "..",
            Operator::DoubleSlash => "//",
            Operator::Less => "<",
            Operator::Greater => ">",
            Operator::BitOr => "|",
            Operator::BitAnd => "&",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Percent => "%",
            Operator::Hash => "#",
            Operator::Tilde => "~",
            Operator::Caret => "^",
        }
    }

}

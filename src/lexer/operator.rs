//! This module supports capturing any operator token
//! Importantly, it does not discriminate between binary and unary operators
//! That will be the job of the parser
//! (Yes, I know I could use a standard EBNF parser, but I want to do it myself)

use lazy_static::lazy_static;
use super::*;

pub enum Operator {
    LogicalOr,
    LogicalAnd,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Inequality,
    Equality,
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
    
}

impl Token for Comment {
    
}

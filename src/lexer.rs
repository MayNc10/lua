//! This module implements lexer functionality
//! It takes in a string representing a lua program and outputs a sequence of lexemes, or tokens

pub mod comment;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod seperator;
pub mod whitespace;

/// This trait isn't used as trait in the code, and will be removed in the future
/// It exists to specify what functions each token type (as represented in the submodules) must provide
trait Token : Sized {
    /// Attempt to parse this token from the input stream
    /// If successful, returns the parsed token and the number of characters consumed
    // Maybe use a different type?
    fn parse(text: &str) -> Option<(Self, u64)>;
    /// Returns the string matched to this token
    fn raw(&self) -> &str; 
} 

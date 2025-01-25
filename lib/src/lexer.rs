//! This module implements lexer functionality
//! It takes in a string representing a lua program and outputs a sequence of lexemes, or tokens

pub mod comment;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod seperator;
pub mod whitespace;

#[cfg(test)]
mod tests;

/// This trait isn't used as trait in the code, and will be removed in the future
/// It exists to specify what functions each token type (as represented in the submodules) must provide
trait Token : Sized {
    /// Attempt to parse this token from the input stream
    /// If successful, returns the parsed token and the number of characters consumed
    // Maybe use a different type?
    fn parse(text: &str) -> Option<(Self, usize)>;
    /// Returns the string matched to this token
    fn raw(&self) -> &str; 
} 

// Make sure this contains all options!
pub enum Lexeme {
    Comment(comment::Comment),
    Keyword(keyword::Keyword),
    StringLiteral(literal::StringLiteral),
    NumericLiteral(literal::NumericLiteral),
    Operator(operator::Operator),
}

pub struct Lexer<'a> {
    text: &'a str,
    index: usize, // Change to some form of span?
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        Lexer { text, index: 0 }
    }

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme;
    
    // Make sure to check for proper ordering!
    fn next(&mut self) -> Option<Lexeme> {
        if let Some((comment, len)) = comment::Comment::parse(self.text) {
            self.index += len;
            Some(Lexeme::Comment(comment))
        }
        else if let Some((kw, len)) = keyword::Keyword::parse(self.text) {
            self.index += len;
            Some(Lexeme::Keyword(kw))
        }
        else if let Some((s, len)) = literal::StringLiteral::parse(self.text) {
            self.index += len;
            Some(Lexeme::StringLiteral(s))
        }
        else if let Some((n, len)) = literal::NumericLiteral::parse(self.text) {
            self.index += len;
            Some(Lexeme::NumericLiteral(n))
        }
        else if let Some((op, len)) = operator::Operator::parse(self.text) {
            self.index += len;
            Some(Lexeme::Operator(op))
        } 
        else {
            None
        }
    }
}

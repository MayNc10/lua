//! This module implements lexer functionality
//! It takes in a string representing a lua program and outputs a sequence of lexemes, or tokens

pub mod assignment; // ?
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
#[derive(Clone, PartialEq, Debug)]
pub enum Lexeme {
    Comment(comment::Comment),
    Keyword(keyword::Keyword),
    StringLiteral(literal::StringLiteral),
    Operator(operator::Operator),
    Assignment(assignment::Assignment),
    NumericLiteral(literal::NumericLiteral),
    Seperator(seperator::Seperator),
    Identifier(identifier::Identifier),
    Whitespace(whitespace::Whitespace),
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
        if self.index >= self.text.len() { return None; }

        let text = &self.text[self.index..];

        if let Some((comment, len)) = comment::Comment::parse(text) {
            self.index += len;
            Some(Lexeme::Comment(comment))
        }
        else if let Some((kw, len)) = keyword::Keyword::parse(text) {
            self.index += len;
            Some(Lexeme::Keyword(kw))
        }
        else if let Some((s, len)) = literal::StringLiteral::parse(text) {
            self.index += len;
            Some(Lexeme::StringLiteral(s))
        }
        // parse op before numbers in order to not consume +/-
        else if let Some((op, len)) = operator::Operator::parse(text) {
            self.index += len;
            Some(Lexeme::Operator(op))
        } 
        else if let Some((a, len)) = assignment::Assignment::parse(text) {
            self.index += len;
            Some(Lexeme::Assignment(a))
        } 
        else if let Some((n, len)) = literal::NumericLiteral::parse(text) {
            self.index += len;
            Some(Lexeme::NumericLiteral(n))
        }
        else if let Some((sep, len)) = seperator::Seperator::parse(text) {
            self.index += len;
            Some(Lexeme::Seperator(sep))
        }
        else if let Some((ident, len)) = identifier::Identifier::parse(text) {
            self.index += len;
            Some(Lexeme::Identifier(ident))
        }
        else if let Some((wsp, len)) = whitespace::Whitespace::parse(text) {
            self.index += len;
            Some(Lexeme::Whitespace(wsp))
        }
        
        else {
            None
        }
    }
}

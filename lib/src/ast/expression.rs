use crate::lexer::Lexer;

pub trait Expression {}
pub struct Literal {}
pub struct TableExpression {}
pub struct BinaryExpression {}
pub struct UnaryExpression {}

pub fn parse_expression(lex: &mut Lexer) -> Option<Box<dyn Expression>> {
    // parse literal
    // parse binary expression
    todo!()
}
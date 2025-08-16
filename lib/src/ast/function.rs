use crate::{ast::Block, lexer::identifier::Identifier};

#[derive(Clone)]
pub struct Function {
    pub args: Vec<Identifier>,
    pub code: Option<Block>,
}
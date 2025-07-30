use std::fmt::Display;

use crate::lexer::Lexer;

/// I'm making this a trait for right now, obviously when we want to speed it up it can be made an enum
pub trait AstNode : Display {
    fn walk(); // does nothing bc we don't have an intepreter
}

pub mod statement;
pub mod expression;

// Maybe move these to a submodule?

#[derive(Debug)]
pub struct Block {
    statements: Vec<Box<dyn statement::Statement>>
}

impl Block {
    pub fn empty() -> Block {
        Block { statements: Vec::new() }
    }

    fn initial(base: Box<dyn statement::Statement>) -> Block {
        Block { statements: vec![base] }
    }

    pub fn push_statement(&mut self, st: Box<dyn statement::Statement>) {
        self.statements.push(st);
    }

    pub fn parse(lex: &mut Lexer) -> Option<Block> {
        // try to parse at least one statement
        if let Some(base) = statement::parse_statement(lex) {
            let mut block = Block::initial(base);
            while let Some(st) = statement::parse_statement(lex) {
                block.push_statement(st);
            }
            Some(block)
        }
        else { None }
    } 
}


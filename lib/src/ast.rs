use std::{collections::HashMap, fmt::Display};

use crate::{ast::context::Ctx, lexer::{identifier::Identifier, keyword::Keyword, seperator, Lexeme, Lexer}, value::Value};

/// I'm making this a trait for right now, obviously when we want to speed it up it can be made an enum
pub trait AstNode : Display {
    fn walk(); // does nothing bc we don't have an intepreter
}

pub mod context;
pub mod function;
pub mod statement;
pub mod expression;

// Maybe move these to a submodule?

#[derive(Clone)]
pub struct Block {
    statements: Vec<statement::Statement>
}

impl Block {
    pub fn empty() -> Block {
        Block { statements: Vec::new() }
    }

    fn initial(base: statement::Statement) -> Block {
        Block { statements: vec![base] }
    }

    pub fn push_statement(&mut self, st: statement::Statement) {
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

    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        println!("{tabs}Block: [");
        for st in &self.statements {
            st.print_tree(depth + 1);
        }
        println!("{tabs}]");
    }

    pub fn walk(&self, ctx: &mut Ctx) {
        ctx.enter_block();
        for st in &self.statements {
            st.walk(ctx);
            if ctx.did_return() {
                ctx.leave_block_noreturn();
                return;
            }
        }
        ctx.leave_block_noreturn();
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Block: [")?;
        for st in &self.statements {
            writeln!(f, "\t{};", st)?;
        }
        writeln!(f, "]")
    }
}

pub fn parse_comma_list<T, F: Fn(&mut Lexer) -> Option<T>, P: Fn(&Lexeme) -> bool>
    (lex: &mut Lexer, parse_func: F, predicate: P) -> Option<Vec<T>> 
{
    let mut items = Vec::new();
    // parse items
    while let Some(lexeme) = lex.clone().peekable().peek() && !predicate(lexeme) {
        items.push(parse_func(lex)?);
        if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
            lex.next();
        }
    }

    assert!(predicate(&lex.next().unwrap()));
    Some(items)
}

pub fn parse_paren_list<T, F: Fn(&mut Lexer) -> Option<T>>(lex: &mut Lexer, parse_func: F) -> Option<Vec<T>> {
    parse_comma_list(lex, parse_func, |lexeme| lexeme == &Lexeme::Seperator(seperator::Seperator::CloseParen))
}

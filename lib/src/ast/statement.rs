use std::fmt::Debug;

use crate::{ast::{expression::{parse_expression, Expression}, Block}, lexer::{self, identifier::Identifier, seperator, Lexeme, Lexer}};

pub trait Statement : Debug {}
pub struct EmptyStatement {}

#[derive(Debug)]
pub struct Assignment {
    ident: Identifier,
    exp: Box<dyn Expression>, // maybe use the enum instead??
}
impl Statement for Assignment {

}
pub struct WhileStatement {}
pub struct RepeatStatement {}

#[derive(Debug)]
pub struct Conditional {
    cases: Vec<(Box<dyn Expression>, Option<Block>)>,
    fallback: Option<Block>,
}
impl Statement for Conditional {

}
pub struct Goto {}
pub struct Label {}
pub struct ForStatement {}

#[derive(Debug)]
pub struct FunctionCall {
    name: Identifier,
    args: Vec<Box<dyn Expression>>,
}
impl Statement for FunctionCall {

}

#[derive(Debug)]
pub struct MethodCall {
    obj_name: Identifier,
    method: Identifier,
    args: Vec<Box<dyn Expression>>,
}
impl Statement for MethodCall {

}

#[derive(Debug)]
pub struct FunctionDef {
    name: Identifier,
    args: Vec<Identifier>,
    code: Option<Block>,
}
impl Statement for FunctionDef {

}
pub struct Break {} // ?

#[derive(Debug)]
pub struct Return {
    val: Option<Box<dyn Expression>>,
} // ?
impl Statement for Return {

}

pub fn parse_statement(lex: &mut Lexer) -> Option<Box<dyn Statement>> {
    //println!("Parse statement");
    // parse assignment
    let mut dup_lex = lex.clone();
    if let Some(Lexeme::Identifier(i)) = lex.next() 
        && let Some(Lexeme::Assignment(assign)) = lex.next()
        && let Some(exp) = parse_expression(lex)
    {
        println!("parsed assignment!");
        return Some(Box::new(Assignment {ident: i, exp}));
    }
    *lex = dup_lex;
    // parse if
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::If)) = lex.next() {
        let mut cases = Vec::new();
        let mut fallback = None;
        let test = parse_expression(lex).unwrap();
        println!("parsed if test");
        let then_kw = lex.next();
        let code = Block::parse(lex);
        cases.push((test, code));
        eprintln!("parsed main if block");
        loop {
            match lex.next() {
                Some(Lexeme::Keyword(lexer::keyword::Keyword::Elseif)) => {
                    let new_test = parse_expression(lex).unwrap();
                    let then_kw = lex.next();
                    let new_code = Block::parse(lex);
                    cases.push((new_test, new_code));
                },
                Some(Lexeme::Keyword(lexer::keyword::Keyword::Else)) => {
                    eprintln!("parsing else");
                    let new_code = Block::parse(lex);
                    fallback = new_code;
                    eprintln!("parsed else");
                    break
                },

                Some(Lexeme::Keyword(lexer::keyword::Keyword::End)) => {
                    println!("reached end kw!");
                    break
                },
                other => {
                    panic!("syntax error parsing if statement, next was {:?}", other);
                }
            }
        }
        lex.next(); //end
        return Some(Box::new(Conditional { cases, fallback }))
    }                                                                                                                                                                                                                               
    *lex = dup_lex;
    // parse functioncall
    if let Some(Lexeme::Identifier(name)) = lex.next()
        && let Some(Lexeme::Seperator(seperator::Seperator::OpenParen)) = lex.next()
    {
        // parse args
        let mut args = Vec::new();
        // THIS IS DUPLICATED CODE FROM EXPRESSION IDENT PARSING
        while lex.clone().peekable().peek() != Some(&Lexeme::Seperator(seperator::Seperator::CloseParen)) {
            let arg = parse_expression(lex).unwrap();
            args.push(arg);
            if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
                lex.next();
            }
        }
        lex.next(); 
        println!("Parsed function call, func name {:?}", name);
        return Some(Box::new(FunctionCall { name, args }));
    }
    *lex = dup_lex;
    // parse method call
    if let Some(Lexeme::Identifier(obj)) = lex.next()
        && let Some(Lexeme::Seperator(seperator::Seperator::Dot)) = lex.next()
        && let Some(Lexeme::Identifier(method)) = lex.next()
        && let Some(Lexeme::Seperator(seperator::Seperator::OpenParen)) = lex.next()
    {
        // parse args
        let mut args = Vec::new();
        // THIS IS DUPLICATED CODE FROM EXPRESSION IDENT PARSING
        while lex.clone().peekable().peek() != Some(&Lexeme::Seperator(seperator::Seperator::CloseParen)) {
            let arg = parse_expression(lex).unwrap();
            args.push(arg);
            if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
                lex.next();
            }
        }
        lex.next(); 
        println!("Parsed method call, obj name {:?}, method name {:?}", obj, method);
        return Some(Box::new(MethodCall { obj_name: obj, method, args }));
    }
    *lex = dup_lex;
    // parse functiondef                                                                                                        
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Function)) = lex.next() 
        && let Some(Lexeme::Identifier(name)) = lex.next()
    {
        let oparen = lex.next();
        // just one for now lol
        let mut args = Vec::new();
        if let Some(Lexeme::Identifier(argname)) = lex.next() {
            args.push(argname); 
        }                                                                                                                                                                                                                                                                  
        let cparen: Option<Lexeme> = lex.next();
        let code = Block::parse(lex);
        let end_kw = lex.next();

        println!("parsed function def!");
        return Some(Box::new(FunctionDef { name, args, code }));
    }
    *lex = dup_lex;

    // parse return
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Return)) = lex.next() {

        let val = parse_expression(lex);
        println!("parsed return!");
        return Some(Box::new(Return { val }));
    }
    *lex = dup_lex;
    //eprintln!("{:?}", lex.clone().peekable().peek());
    //todo!()
    None
}
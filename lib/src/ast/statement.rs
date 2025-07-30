use crate::{ast::{expression::parse_expression, Block}, lexer::{self, seperator, Lexeme, Lexer}};

pub trait Statement {}
pub struct EmptyStatement {}
pub struct Assignment {}
impl Statement for Assignment {

}
pub struct WhileStatement {}
pub struct RepeatStatement {}
pub struct Conditional {}
impl Statement for Conditional {

}
pub struct Goto {}
pub struct Label {}
pub struct ForStatement {}
pub struct FunctionCall {}
impl Statement for FunctionCall {

}
pub struct MethodCall {}
impl Statement for MethodCall {

}
pub struct FunctionDef {}
impl Statement for FunctionDef {

}
pub struct Break {} // ?
pub struct Return {} // ?
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
        return Some(Box::new(Assignment {}));
    }
    *lex = dup_lex;
    // parse if
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::If)) = lex.next() {
        let test = parse_expression(lex);
        println!("parsed if test");
        let then_kw = lex.next();
        let code = Block::parse(lex);
        eprintln!("parsed main if block");
        loop {
            match lex.next() {
                Some(Lexeme::Keyword(lexer::keyword::Keyword::Elseif)) => {
                    let new_test = parse_expression(lex);
                    let then_kw = lex.next();
                    let new_code = Block::parse(lex);
                },
                Some(Lexeme::Keyword(lexer::keyword::Keyword::Else)) => {
                    eprintln!("parsing else");
                    let new_code = Block::parse(lex);
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
        return Some(Box::new(Conditional {}))
    }                                                                                                                                                                                                                               
    *lex = dup_lex;
    // parse functioncall
    if let Some(Lexeme::Identifier(i)) = lex.next()
        && let Some(Lexeme::Seperator(seperator::Seperator::OpenParen)) = lex.next()
    {
        // parse args
        // THIS IS DUPLICATED CODE FROM EXPRESSION IDENT PARSING
        while lex.clone().peekable().peek() != Some(&Lexeme::Seperator(seperator::Seperator::CloseParen)) {
            let _ = parse_expression(lex);
            if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
                lex.next();
            }
        }
        lex.next(); 
        println!("Parsed function call, func name {:?}", i);
        return Some(Box::new(FunctionCall {}));
    }
    *lex = dup_lex;
    // parse method call
    if let Some(Lexeme::Identifier(obj)) = lex.next()
        && let Some(Lexeme::Seperator(seperator::Seperator::Dot)) = lex.next()
        && let Some(Lexeme::Identifier(method)) = lex.next()
        && let Some(Lexeme::Seperator(seperator::Seperator::OpenParen)) = lex.next()
    {
        // parse args
        // THIS IS DUPLICATED CODE FROM EXPRESSION IDENT PARSING
        while lex.clone().peekable().peek() != Some(&Lexeme::Seperator(seperator::Seperator::CloseParen)) {
            let _ = parse_expression(lex);
            if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
                lex.next();
            }
        }
        lex.next(); 
        println!("Parsed method call, obj name {:?}, method name {:?}", obj, method);
        return Some(Box::new(MethodCall {}));
    }
    *lex = dup_lex;
    // parse functiondef                                                                                                        
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Function)) = lex.next() {
        let fname = lex.next();  
        let oparen = lex.next();
        // just one for now lol
        let argname = lex.next();                                                                                                                                                                                                                                                                             
        let cparen: Option<Lexeme> = lex.next();
        let code = Block::parse(lex);
        let end_kw = lex.next();

        println!("parsed function def!");
        return Some(Box::new(FunctionDef {}));
    }
    *lex = dup_lex;

    // parse return
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Return)) = lex.next() {

        let exp = parse_expression(lex);
        println!("parsed return!");
        return Some(Box::new(Return {}));
    }
    *lex = dup_lex;
    //eprintln!("{:?}", lex.clone().peekable().peek());
    //todo!()
    None
}
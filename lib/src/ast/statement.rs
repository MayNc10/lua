use std::{collections::HashMap, fmt::{Debug, Display}};

use crate::{ast::{context::Ctx, expression::{parse_expression, Expression}, function::{Function, FunctionCall}, Block}, lexer::{self, identifier::Identifier, seperator, Lexeme, Lexer}, value::{Boolean, Value}};

#[derive(Clone)]
pub struct Assignment {
    ident: Identifier,
    exp: Expression, // maybe use the enum instead??
}

impl Assignment {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}Assignment [ ");
        print!("\t{} = {} ", self.ident, self.exp);
        println!(" ]");
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assignment [ ")?;
        write!(f, "\t{} = {} ", self.ident, self.exp)?;
        write!(f, "]")
    }
}


pub struct WhileStatement {}
pub struct RepeatStatement {}

#[derive(Clone)]
pub struct Conditional {
    cases: Vec<(Expression, Option<Block>)>,
    fallback: Option<Block>,
}

impl Conditional {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        println!("{tabs}Conditional: [");
        for (test, code) in &self.cases {
            println!("{tabs}\tTest: {test}");
            println!("{tabs}\tCode: ");
            if let Some(b) = code {
                b.print_tree(depth + 2);
            } else {
                println!("Nothing");
            }
        }
        println!("{tabs}]");
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Conditional: [")?;
        for (test, code) in &self.cases {
            writeln!(f, "\tTest: {test}")?;
            write!(f, "\t\tCode: ")?;
            if let Some(b) = code {
                writeln!(f, "{b}")?;
            } else {
                writeln!(f, "Nothing")?;
            }
        }
        write!(f, "]")
    }
}

pub struct Goto {}
pub struct Label {}
pub struct ForStatement {}

#[derive(Clone)]
pub struct MethodCall {
    obj_name: Identifier,
    method: Identifier,
    args: Vec<Expression>,
}

impl MethodCall {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}MethodCall [ {}.{}(", self.obj_name, self.method);
        if self.args.len() > 0 {
            for arg in &self.args[0..self.args.len() - 1] {
                print!("{tabs}{arg}, ");
            }
            print!("{tabs}{}", self.args.last().unwrap());
        } else { print!("{tabs}void"); }
        println!("{tabs}) ]");
    }
}

impl Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MethodCall [ {}.{}(", self.obj_name, self.method)?;
        if self.args.len() > 0 {
            for arg in &self.args[0..self.args.len() - 1] {
                write!(f, "{arg}, ")?;
            }
            write!(f, "{}", self.args.last().unwrap())?;
        } else { write!(f, "void")?; }
        write!(f, ") ]")
    }
}

#[derive(Clone)]
pub struct FunctionDef {
    name: Identifier,
    func: Function,
}

impl FunctionDef {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        println!("{tabs}FunctionDef: [");
        println!("{tabs}\tName: {}", self.name);
        print!("{tabs}\tArgs: ");
        if self.func.args.len() > 0 {
            for arg in &self.func.args[0..(self.func.args.len() - 1)] {
                print!("{tabs}{arg}, ");
            }
            println!("{tabs}{}", self.func.args[self.func.args.len() - 1]);
        } else {
            println!("{tabs}(Nothing)");
        }
        println!("{tabs}\tCode: ");
        if let Some(b) = &self.func.code {
            b.print_tree(depth + 2);
        } else {
            println!("{tabs}Nothing");
        }
        println!("{tabs}]");
    }
}

impl Display for FunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FunctionDef: [")?;
        writeln!(f, "\tName: {}", self.name)?;
        write!(f, "\tArgs: ")?;
        if self.func.args.len() > 0 {
            for arg in &self.func.args[0..(self.func.args.len() - 1)] {
                write!(f, "{arg}, ")?;
            }
            writeln!(f, "{}", self.func.args[self.func.args.len() - 1])?;
        } else {
            writeln!(f, "(Nothing)")?;
        }
        write!(f, "\tCode: ")?;
        if let Some(b) = &self.func.code {
            writeln!(f, "{b}")?;
        } else {
            writeln!(f, "Nothing")?;
        }
        write!(f, "]")
    }
}

pub struct Break {} // ?

#[derive(Clone)]
pub struct Return {
    val: Option<Expression>,
} // ?

impl Return {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}Return [ ");
        if let Some(val) = &self.val {
            print!("{val}");
        } else { print!("{tabs}void"); }
        println!(" ]");
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return [ ")?;
        if let Some(val) = &self.val {
            write!(f, "{val}")?;
        } else { write!(f, "void")?; }
        write!(f, " ]")
    }
}

#[derive(Clone)]
pub enum Statement {
    Assignment(Assignment),
    Conditional(Conditional),
    FunctionDef(FunctionDef),
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    Return(Return)
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assignment(a) => { write!(f, "{}", a) },
            Statement::Conditional(cond) => { write!(f, "{}", cond) },
            Statement::FunctionDef(fdef) => { write!(f, "{}", fdef) },
            Statement::FunctionCall(fcall) => { write!(f, "{}", fcall) },
            Statement::MethodCall(mcall) => { write!(f, "{}", mcall) },
            Statement::Return(r) => { write!(f, "{}", r) }
        }
    }
}

impl Statement {
    pub fn print_tree(&self, depth: usize) {
        //let tabs = "\t".repeat(depth);
        match self {
            Statement::Assignment(a) => { a.print_tree(depth) },
            Statement::Conditional(cond) => { cond.print_tree(depth) },
            Statement::FunctionDef(fdef) => { fdef.print_tree(depth) },
            Statement::FunctionCall(fcall) => { fcall.print_tree(depth) },
            Statement::MethodCall(mcall) => { mcall.print_tree(depth) },
            Statement::Return(r) => { r.print_tree(depth) }
        }
    }

    pub fn walk(&self, ctx: &mut Ctx) {
        match self {
            Statement::Assignment(a) => {
                // FIXME: WE ARENT HANDLING LOCALS!
                let val = a.exp.eval(ctx);
                ctx.new_global(a.ident.clone(), val);
            },
            Statement::Conditional(c) => {
                for (exp, block) in &c.cases {
                    match exp.eval(ctx) {
                        Value::Boolean(Boolean::False) | Value::Nil => {},
                        _ => {
                            if let Some(block) = block {
                                block.walk(ctx);
                            }
                            return;
                        }
                    }
                }
                if let Some(block) = &c.fallback {
                    block.walk(ctx);
                }
            },
            Statement::FunctionDef(fdef) => {
                // FIXME: THIS TREATS ALL FUNCTIONS AS GLOBALS
                ctx.new_global(fdef.name.clone(), Value::Function(fdef.func.clone()));
            },
            Statement::FunctionCall(fcall) => {
                fcall.call(ctx);
            },
            Statement::Return(r) => {
                let rv = r.val.as_ref().map(|exp| exp.eval(ctx));
                ctx.ret(rv);
            }
            _ => todo!()
        }
    }
}

pub fn parse_statement(lex: &mut Lexer) -> Option<Statement> {
    //println!("Parse statement");
    // parse assignment
    let mut dup_lex = lex.clone();
    if let Some(Lexeme::Identifier(i)) = lex.next() 
        && let Some(Lexeme::Assignment(assign)) = lex.next()
        && let Some(exp) = parse_expression(lex)
    {
        println!("parsed assignment!");
        return Some(Statement::Assignment(Assignment {ident: i, exp}));
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
        return Some(Statement::Conditional(Conditional { cases, fallback }))
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
        return Some(Statement::FunctionCall(FunctionCall::new(name, args)));
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
        return Some(Statement::MethodCall(MethodCall { obj_name: obj, method, args }));
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
        return Some(Statement::FunctionDef(FunctionDef { name, func: Function { args, code } }));
    }
    *lex = dup_lex;

    // parse return
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Return)) = lex.next() {

        let val = parse_expression(lex);
        println!("parsed return!");
        return Some(Statement::Return(Return { val }));
    }
    *lex = dup_lex;
    //eprintln!("{:?}", lex.clone().peekable().peek());
    //todo!()
    None
}
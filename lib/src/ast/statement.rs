use std::{collections::HashMap, fmt::{Debug, Display}};

use crate::{ast::{context::Ctx, expression::{parse_expression, Expression}, function::{Function, FunctionCall, MethodCall}, parse_paren_list, Block}, lexer::{self, identifier::Identifier, seperator, Lexeme, Lexer}, value::{flatten_values, Boolean, Value}};

#[derive(Clone)]
pub struct Assignment {
    idents: Vec<Identifier>,
    exps: Vec<Expression>,
    local: bool
}

impl Assignment {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}Assignment [ ");
        for ident in &self.idents {
            print!("{ident} ");
        }
        print!("= ");
        for exp in &self.exps {
            print!("{exp} ");
        }
        println!("]");
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assignment [ ")?;
        for ident in &self.idents {
            write!(f, "{ident} ")?;
        }
        write!(f, "= ")?;
        for exp in &self.exps {
            write!(f, "{exp} ")?;
        }
        writeln!(f, "]")
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
        if let Some(fb) = &self.fallback {
            println!("{tabs}\tFallback: ");
            fb.print_tree(depth + 2);
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
    vals: Vec<Expression>,
} // ?

impl Return {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}Return [ ");
        for val in &self.vals {
            print!("{val} ");
        }
        println!("]");
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return [ ")?;
        for val in &self.vals {
            write!(f, "{val} ")?;
        }
        writeln!(f, "]")
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

                // evaulate expressions even if unused
                let mut values = flatten_values(a.exps.iter().map(|e| e.eval(ctx)).collect::<>()).into_iter();
                for ident in &a.idents {
                    ctx.new_global(ident.clone(), values.next().unwrap_or(Value::Nil));
                }
            },
            Statement::Conditional(c) => {
                for (exp, block) in &c.cases {
                    let res = exp.eval(ctx).as_bool();
                    if res {
                        if let Some(block) = block {
                            block.walk(ctx);
                        }
                        return;
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
                // FIXME
                let rv = r.vals.iter().map(|exp| exp.eval(ctx)).collect();
                ctx.ret(rv);
            }
            Statement::MethodCall(mcall) => {
                mcall.call(ctx);
            }
        }
    }
}

pub fn parse_statement(lex: &mut Lexer) -> Option<Statement> {
    //println!("Parse statement");
    // parse assignment
    let dup_lex = lex.clone();

    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Local)) = lex.next() {
        if let Some(Statement::Assignment(mut assign)) = parse_statement(lex) {
            assign.local = true;
            return Some(Statement::Assignment(assign));
        } else { panic!("Local without following assignment") }
    }

    *lex = dup_lex;
    if let Some(Lexeme::Identifier(i)) = lex.next() 
    {
        // try to parse more idents
        let mut idents = vec![i];
        let mut failed = false;
        while let Some(lx) = lex.next() && !matches!(lx, Lexeme::Assignment(_)) {
            if !matches!(lx, Lexeme::Seperator(seperator::Seperator::Comma)) {
                failed = true;
                break;
            }
            if let Some(Lexeme::Identifier(ident)) = lex.next() {
                idents.push(ident);
            } else { failed = true; break; }
        }
        if !failed { 
            let mut exps = Vec::new();
            if let Some(first_e) = parse_expression(lex) {
                // assignments have to have at least one expression
                exps.push(first_e);
                while lex.clone().next() == Some(Lexeme::Seperator(seperator::Seperator::Comma)) {
                    lex.next();
                    exps.push(parse_expression(lex).unwrap());
                }
                //println!("parsed assignment!");
                return Some(Statement::Assignment(Assignment {idents, exps, local: false}));
            }
        }
    }
    *lex = dup_lex;
    // parse if
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::If)) = lex.next() {
        let mut cases = Vec::new();
        let mut fallback = None;
        let test = parse_expression(lex).unwrap();
        //println!("parsed if test");
        let then_kw = lex.next();
        let code = Block::parse(lex);
        cases.push((test, code));
        // FIXME: shitty stupid hack
        let mut hit_end = false;
        loop {
            match lex.next() {
                Some(Lexeme::Keyword(lexer::keyword::Keyword::Elseif)) => {
                    let new_test = parse_expression(lex).unwrap();
                    let then_kw = lex.next();
                    let new_code = Block::parse(lex);
                    cases.push((new_test, new_code));
                },
                Some(Lexeme::Keyword(lexer::keyword::Keyword::Else)) => {
                    //eprintln!("parsing else");
                    let new_code = Block::parse(lex);
                    fallback = new_code;
                    //eprintln!("parsed else");
                    break
                },

                Some(Lexeme::Keyword(lexer::keyword::Keyword::End)) => {
                    hit_end = true;
                    break
                },
                other => {
                    panic!("syntax error parsing if statement, then kw was {:?}, next was {:?}", then_kw, other);
                }
            }
        }
        if !hit_end { lex.next();  }
        return Some(Statement::Conditional(Conditional { cases, fallback }))
    }                                                                                                                                                                                                                               
    *lex = dup_lex;
    // parse functioncall
    if let Some(fcall) = FunctionCall::parse(lex)
    {
        return Some(Statement::FunctionCall(fcall));
    }
    *lex = dup_lex;
    // parse method call
    if let Some(mcall) = MethodCall::parse(lex) {
        return Some(Statement::MethodCall(mcall));
    }
    *lex = dup_lex;
    // parse functiondef                                                                                                        
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Function)) = lex.next() 
        && let Some(Lexeme::Identifier(name)) = lex.next()
    {
        let oparen = lex.next();
        
        let args = parse_paren_list(lex, |l| 
            if let Some(Lexeme::Identifier(ident)) = l.next() {
                Some(ident)
            } else { None }
        ).unwrap();                                                                                                                                                                                                                                               
        let code = Block::parse(lex);
        let end_kw = lex.next();
        let fdef = Statement::FunctionDef(FunctionDef { name, func: Function { args, code } });
        assert_eq!(end_kw, Some(Lexeme::Keyword(lexer::keyword::Keyword::End)));

        //println!("parsed function def!");
        return Some(fdef);
    }
    *lex = dup_lex;

    // parse return
    if let Some(Lexeme::Keyword(lexer::keyword::Keyword::Return)) = lex.next() {

        let mut vals = Vec::new();
        if let Some(exp) = parse_expression(lex) {
            vals.push(exp);
            while lex.clone().next() == Some(Lexeme::Seperator(seperator::Seperator::Comma)) {
                lex.next();
                vals.push(parse_expression(lex).unwrap());
            }
        }
        return Some(Statement::Return(Return { vals }));
    }
    *lex = dup_lex;
    //eprintln!("{:?}", lex.clone().peekable().peek());
    //todo!()
    None
}
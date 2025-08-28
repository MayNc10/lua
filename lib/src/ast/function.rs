use std::fmt::{Debug, Display};

use crate::{ast::{context::Ctx, expression::{parse_expression, Expression}, parse_comma_list, parse_paren_list, Block}, lexer::{identifier::Identifier, seperator::Seperator, Lexeme, Lexer}, value::Value};

#[derive(Clone)]
pub struct Function {
    pub args: Vec<Identifier>,
    pub code: Option<Block>,
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    name: Identifier,
    args: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(name: Identifier, args: Vec<Expression>) -> FunctionCall {
        FunctionCall { name, args }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn args(&self) -> &Vec<Expression> {
        &self.args
    }

    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}FunctionCall [ {}(", self.name);
        if self.args.len() > 0 {
            for arg in &self.args[0..self.args.len() - 1] {
                print!("{tabs}{arg}, ");
            }
            print!("{tabs}{}", self.args.last().unwrap());
        } else { print!("{tabs}void"); }
        println!("{tabs}) ]");
    }

    pub fn parse(lex: &mut Lexer) -> Option<FunctionCall> {
        if let Some(Lexeme::Identifier(ident)) = lex.next() 
        && let Some(Lexeme::Seperator(Seperator::OpenParen)) = lex.next()
        {
            //println!("resolving function call");
            let exps = parse_paren_list(lex, parse_expression).unwrap();
            Some(FunctionCall::new(ident.clone(), exps))
        } else { None }
    }

    pub fn call(&self, ctx: &mut Ctx) -> Value {
        // FIXME: REGISTER AS A GLOBAL FUNCTION
        if self.name.0 == "print" {
            for val in self.args.iter().map(|e| e.eval(ctx)) {
                match val {
                    Value::String(s) => print!("{s}"),
                    Value::Number(n) => print!("{n}"),
                    _ => todo!()
                }
                print!("\t")
            }
            println!();
            return Value::Nil;
        }
        match ctx.get_var(&self.name) {
            Some(Value::Function(fcode)) => {
                // resolve argument expressions
                let arg_vals= self.args.iter().map(|e| e.eval(ctx)).collect::<Vec<_>>();
                let mut val_iter = arg_vals.into_iter();
                ctx.enter_block();
                // add new locals
                for arg in &fcode.args {
                    eprintln!("Adding local {} with value {:?}", arg.0, val_iter.clone().next());
                    ctx.new_local(arg.clone(), val_iter.next().unwrap_or(Value::Nil));
                }
                if let Some(code) = fcode.code {
                    code.walk(ctx);
                }
                ctx.leave_block().unwrap_or(Value::Nil)
            },
            // FIXME
            _ => { panic!("function not defined, handle this error gracefully!") }
        }
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionCall [ {}(", self.name)?;
        if self.args.len() > 0 {
            for arg in &self.args[0..self.args.len() - 1] {
                write!(f, "{arg}, ")?;
            }
            write!(f, "{}", self.args.last().unwrap())?;
        } else { write!(f, "void")?; }
        write!(f, ") ]")
    }
}

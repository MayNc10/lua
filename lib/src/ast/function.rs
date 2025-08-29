use std::{fmt::{Debug, Display}, io};

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


#[derive(Clone, Debug)]
pub struct MethodCall {
    obj: Identifier,
    method: Identifier,
    args: Vec<Expression>,
}

impl MethodCall {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        print!("{tabs}MethodCall [ {}.{}(", self.obj, self.method);
        if self.args.len() > 0 {
            for arg in &self.args[0..self.args.len() - 1] {
                print!("{tabs}{arg}, ");
            }
            print!("{tabs}{}", self.args.last().unwrap());
        } else { print!("{tabs}void"); }
        println!("{tabs}) ]");
    }

    pub fn obj_name(&self) -> &str {
        self.obj.0.as_str()
    }

    pub fn method_name(&self) -> &str {
        self.method.0.as_str()
    }

    pub fn parse(lex: &mut Lexer) -> Option<MethodCall> {
        println!("parsing method call!");
        if let Some(Lexeme::Identifier(obj)) = lex.next()
            && let Some(Lexeme::Seperator(Seperator::Dot)) = lex.next()
            && let Some(fcall) = FunctionCall::parse(lex)
        {
            
            println!("Parsed method call, obj name {:?}, method name {:?}", obj, fcall.name);
            return Some(MethodCall { obj, method: fcall.name, args: fcall.args });
        } else { None }
    }

    pub fn call(&self, ctx: &mut Ctx) -> Value {
        eprintln!("Calling method {} on object {}", self.method.0, self.obj.0);
        if self.obj.0 == "io" && self.method.0 == "read" {
            let mut buf = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buf).unwrap();

            // check args
            if let Some(a) = self.args.first() 
            && let Value::String(s) = a.eval(ctx) 
            && &s.as_str()[0..1] == "n" 
            {
                Value::Number(buf.trim().parse().unwrap())
            } else {
                Value::String(buf)   
            }    
        }

        else if self.obj.0 == "io" && self.method.0 == "write" {
            for arg in &self.args {
                let v = arg.eval(ctx);
                match v {
                    Value::String(s) => print!("{s}"),
                    Value::Number(n) => print!("{n}"),
                    _ => print!("{:?}", v)
                }
            }
            Value::Nil
        }

        else if self.obj.0 == "math" && self.method.0 == "abs" {
            if let Some(Value::Number(n)) = self.args.first().map(|e| e.eval(ctx)) {
                Value::Number(n.abs())
            } else { panic!() }
        }

        else {
            panic!("Method stuff is hacked together rn!");
        }
    }
}

impl Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MethodCall [ {}.{}(", self.obj, self.method)?;
        if self.args.len() > 0 {
            for arg in &self.args[0..self.args.len() - 1] {
                write!(f, "{arg}, ")?;
            }
            write!(f, "{}", self.args.last().unwrap())?;
        } else { write!(f, "void")?; }
        write!(f, ") ]")
    }
}

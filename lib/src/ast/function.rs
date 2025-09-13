use std::{fmt::{Debug, Display}, io};

use crate::{ast::{context::Ctx, expression::{parse_expression, Expression}, parse_comma_list, parse_paren_list, Block}, builtins, lexer::{identifier::Identifier, seperator::Seperator, Lexeme, Lexer}, value::{flatten_values, Value}};

#[derive(Clone)]
pub struct LuaFunction {
    pub args: Vec<Identifier>,
    pub code: Option<Block>,
}

#[derive(Clone)]
pub enum Function {
    LuaFunction(LuaFunction),
    Builtin(fn(&Vec<Value>) -> Vec<Value>),
}

impl Function {
    pub fn call(&self, args: &Vec<Expression>, ctx: &mut Ctx) -> Value {
        let arg_vals = args.iter().map(|e| e.eval(ctx)).collect::<Vec<_>>();
        let mut rvs = flatten_values(
        match self {
                Function::LuaFunction(lfunc) => {
                    // resolve argument expressions
                    let mut val_iter = arg_vals.into_iter();
                    // Note: there's some weird behavior with this one
                    // In order to make lexical scoping work, block entry automically makes the context enter a block
                    // This means, when we call a function, we actually enter the block twice - once here, once when calling code.walk(ctx)
                    // FIXME!

                    // this entry should actually add a new function gate
                    // locals in larger scopes can be accessed, until you cross the function barrier.
                    ctx.enter_block();
                    // add new locals
                    for arg in &lfunc.args {
                        ctx.new_local(arg.clone(), val_iter.next().unwrap_or(Value::Nil));
                    }
                    if let Some(code) = &lfunc.code {
                        code.walk(ctx);
                    }
                    ctx.leave_block()
                    
                }
                Function::Builtin(bfunc) => {
                    bfunc(&arg_vals)
                }
        });   
        if rvs.is_empty() { Value::Nil }
        else if rvs.len() == 1 { rvs.pop().unwrap() }
        else { Value::RetVals(rvs) }
    }
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
            let exps = parse_paren_list(lex, parse_expression);
            if exps.is_none() {
                eprintln!("Didn't parse paren functioncall list properly!");
                for l in lex {
                    eprintln!("\tNext lex: {l:?}");
                }
                todo!();
             }
            let exps = exps.unwrap();
            Some(FunctionCall::new(ident.clone(), exps))
        } 
        else { None }
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
                fcode.call(&self.args, ctx)
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
    obj: Box<Expression>,
    method: Identifier,
    args: Vec<Expression>,
}

impl MethodCall {
    /* 
    pub fn parse(lex: &mut Lexer) -> Option<MethodCall> {
        let dup_lex = lex.clone();
        if let Some(obj) = parse_expression(lex)
            && let Some(Lexeme::Seperator(Seperator::Dot)) = lex.next()
            && let Some(fcall) = FunctionCall::parse(lex)
        {
            return Some(MethodCall { obj: Box::new(obj), method: fcall.name, args: fcall.args });
        }
        *lex = dup_lex; 
        if let Some(obj) = parse_expression(lex)
            && let Some(Lexeme::Seperator(Seperator::Colon)) = lex.next()
            && let Some(mut fcall) = FunctionCall::parse(lex)
        {
            let mut args = vec![obj.clone()];
            args.append(&mut fcall.args);
            return Some(MethodCall { obj: Box::new(obj), method: fcall.name, args });
        } 
        else { None }
    }*/

    pub fn call(&self, ctx: &mut Ctx) -> Value {
        /*if self.obj.0 == "io" && self.method.0 == "read" {
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
        } */

        if self.obj_name() == Some("math") && self.method.0 == "abs" {
            if let Some(Value::Number(n)) = self.args.first().map(|e| e.eval(ctx)) {
                Value::Number(n.abs())
            } else { panic!() }
        }

        else if self.obj_name() == Some("string") && self.method.0 == "format" {
            let s = self.args[0].eval(ctx).as_string().expect("format string arg wasnt string");
            let vals = self.args[1..].iter().map(|e| e.eval(ctx)).collect();
            Value::String(builtins::string::format(&s, &vals))
        }

        else {
            if let Value::Table(t) = self.obj.eval(ctx) {
                if let Some(Value::Function(f)) = t.borrow().get(&Value::String(self.method.0.clone())) {
                    f.call(&self.args, ctx)
                } else { panic!("method not found in object") }
            } else { panic!("method object not found") }
        }
    }

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

    pub fn obj_name(&self) -> Option<&str> {
        match &*self.obj {
            Expression::Identifier(i) => Some(i.0.as_str()),
            _ => None,
        }
    }

    pub fn method_name(&self) -> &str {
        self.method.0.as_str()
    }

    pub fn new(obj: Expression, method: Identifier, args: Vec<Expression>) -> MethodCall {
        MethodCall { obj: Box::new(obj), method, args }
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

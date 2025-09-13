use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{ast::{context::Ctx, expression::{parse_expression, Expression}}, lexer::{assignment::Assignment, identifier::Identifier, seperator::Seperator, Lexeme, Lexer}, value::Value};

#[derive(Clone, PartialEq)]
struct ValWrapper {
    inner: Value
}

impl From<Value> for ValWrapper {
    fn from(value: Value) -> Self {
        ValWrapper { inner: value }
    }
}

impl Into<Value> for ValWrapper {
    fn into(self) -> Value {
        self.inner
    }
}

// FIXME: SUCKS!
impl std::hash::Hash for ValWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.inner {
            Value::Nil => {
                state.write_u8(1);
            },
            Value::Boolean(b) => {
                state.write_u8(2);
                b.hash(state);
            },
            Value::Number(n) => {
                state.write_u8(3);
                if n.is_nan() { panic!("hashed value was nan") }
                n.to_bits().hash(state);
            },
            Value::String(s) => {
                state.write_u8(4);
                s.hash(state);
            },
            Value::Userdata => {
                state.write_u8(5);
            },
            Value::Function(f) => {
                state.write_u8(6);
                todo!()
            },
            Value::Thread => {
                state.write_u8(7);
            },
            Value::Table(tb) => {
                state.write_u8(8);
                Rc::as_ptr(&tb).hash(state);
            },
            Value::RetVals(rv) => {
                state.write_u8(9);
                let rv_wrap = rv.iter().map(|v| ValWrapper::from(v.clone())).collect::<Vec<_>>();
                rv_wrap.hash(state);
            },
        }
    }
}

// BAD! VERY VERY BAD!
impl Eq for ValWrapper {}


#[derive(Clone)]
pub struct Table {
    inner: HashMap<ValWrapper, Value>
}

impl Table {
    pub fn new() -> Rc<RefCell<Table>> {
        Rc::new(RefCell::new(Table { inner: HashMap::new() }))
    }

    pub fn insert(&mut self, key: &Value, val: Value) {
        if *key == Value::Nil {
            panic!("key shouldnt be null!")
        }

        if val == Value::Nil {
            // FIXME: SUCKS, but we can fix
            self.inner.remove(&ValWrapper::from(key.clone()));
        }
        else {
            // FIXME
            self.inner.insert(ValWrapper::from(key.clone()), val);
        }
    }

    pub fn get(&self, key: &Value) -> Option<Value> {
        if *key == Value::Nil {
            panic!("key shouldnt be null!")
        }

        self.inner.get(&ValWrapper::from(key.clone())).map(|v| v.clone())
    }
}

#[derive(Clone, Debug)]
pub struct TableConstructor {
    record: Vec<(Identifier, Expression)>,
    sequence: Vec<Expression>,
    general: Vec<(Expression, Expression)>,
}

impl TableConstructor {
    pub fn parse(lex: &mut Lexer) -> Option<TableConstructor> {
        let mut record = Vec::new();
        let mut sequence = Vec::new();
        let mut general = Vec::new();

        if lex.next() == Some(Lexeme::Seperator(Seperator::OpenCurly)) {
            while lex.clone().next() != Some(Lexeme::Seperator(Seperator::CloseCurly)) {
                if lex.clone().next().is_none() { return None }

                // try to parse expression (won't capture record assigment)
                let lex_clone = lex.clone();
                if let Some(exp) = parse_expression(lex) {
                    sequence.push(exp);
                    continue;
                }
                *lex = lex_clone;
                if let Some(Lexeme::Identifier(ident)) = lex.next() 
                && lex.next() == Some(Lexeme::Assignment(Assignment {})) 
                && let Some(exp) = parse_expression(lex)
                {
                    // parsing record
                    record.push((ident, exp));
                    continue;
                }
                *lex = lex_clone;
                if lex.next() == Some(Lexeme::Seperator(Seperator::OpenBracket))
                && let Some(lhs) = parse_expression(lex)
                && lex.next() == Some(Lexeme::Seperator(Seperator::CloseBracket))
                && lex.next() == Some(Lexeme::Assignment(Assignment {})) 
                && let Some(rhs) = parse_expression(lex)
                {
                    general.push((lhs, rhs));
                }
            } 
            lex.next();

            Some(TableConstructor { record, sequence, general })
        } else { None }
    }
}

#[derive(Clone, Debug)]
pub enum TableAccess {
    DotAccess(Box<Expression>, Identifier)
}

impl TableAccess {
    /* 
    pub fn parse(lex: &mut Lexer) -> Option<TableAccess> {
        if let Some(Lexeme::Identifier(tname)) = lex.next()
            && lex.next() == Some(Lexeme::Seperator(Seperator::Dot))
            && let Some(Lexeme::Identifier(vname)) = lex.next() 
        {
            Some(TableAccess::DotAccess( tname, vname))
        }
        else { None }
    } 
    */
    pub fn new_dot(obj: Expression, field: Identifier) -> TableAccess {
        TableAccess::DotAccess(Box::new(obj), field)
    }
}

#[derive(Clone, Debug)]
pub enum TableAssign {
    DotAssign(Identifier, Identifier, Expression)
}

impl TableAssign {
    pub fn parse(lex: &mut Lexer) -> Option<TableAssign> {
        if let Some(Lexeme::Identifier(tname)) = lex.next()
            && lex.next() == Some(Lexeme::Seperator(Seperator::Dot))
            && let Some(Lexeme::Identifier(vname)) = lex.next() 
            && lex.next() == Some(Lexeme::Assignment(Assignment {  })) 
            && let Some(exp) = parse_expression(lex) 
        {
            Some(TableAssign::DotAssign(tname, vname, exp))
        } else { None }
    }

    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        println!("{tabs}Table Assignment [");
        match self {
            TableAssign::DotAssign(obj, field, exp) => {
                println!("{tabs}\ttable: {}", obj.0);
                println!("{tabs}\tfield: {}", field.0);
                println!("{tabs}\tvalue: {}", exp);
            }
        }
        println!("{tabs}]");
    }

    pub fn walk(&self, ctx: &mut Ctx) {
        todo!()
    }
}

impl Display for TableAssign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table Assignment [")?;
        match self {
            TableAssign::DotAssign(obj, field, exp) => {
                writeln!(f, "\ttable: {}", obj.0)?;
                writeln!(f, "\tfield: {}", field.0)?;
                writeln!(f, "\tvalue: {}", exp)?;
            }
        }
        write!(f, "]")
    }
}
use std::fmt::Debug;

use crate::ast::{function::Function, Block};

pub mod meta;

#[derive(Clone)]
pub enum Value {
    Nil,
    Boolean(Boolean),
    Number(f64),
    String(String),
    Userdata,
    Function(Function),
    Thread,
    Table,
}

impl Value {
    pub fn val_str(&self) -> &str {
        match self {
            Value::Nil => "Nil",
            Value::Boolean(_) => "Boolean",
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Userdata => "Userdata",
            Value::Function(_) => "Function",
            Value::Thread => "Thread",
            Value::Table => "Table",
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::String(s) => s.trim().parse().ok(),
            _ => None
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Nil | Value::Boolean(Boolean::False) => false,
            _ => true,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value [ ")?;
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Boolean(b) => write!(f, "Bool( {b:?} )"),
            Value::Number(n) => write!(f, "Number( {n} )"),
            Value::String(s) => write!(f, "String( {s} )"),
            Value::Userdata => write!(f, "Userdata"),
            Value::Function(func) => write!(f, "Function", ),
            Value::Thread => write!(f, "Thread"),
            Value::Table => write!(f, "Table"),
        }?;
        write!(f, " ]")
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Userdata, Value::Userdata) => todo!(),
            (Value::Function(f1), Value::Function(f2)) => todo!(),
            (Value::Thread, Value::Thread) => todo!(),
            (Value::Table, Value::Table) => todo!(),
            _ => false,
        }
    }
}

// just putting this here bc its so simple
// FIXME: why is this not just a bool lmao?

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boolean {
    True, 
    False,
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        if value {
            Boolean::True
        } else { Boolean::False }
    }
}
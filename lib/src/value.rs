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
    // fixme?
    RetVals(Vec<Value>),
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
            Value::RetVals(_) => "Multiple return values"
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::String(s) => s.trim().parse().ok(),
            Value::RetVals(rv) => rv.first().and_then(|v| v.as_number()),
            _ => None
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Nil | Value::Boolean(Boolean::False) => false,
            _ => true,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        }
    }
}

pub fn flatten_values(vals: Vec<Value>) -> Vec<Value> {
    let mut flat = Vec::with_capacity(vals.len());
    for val in vals {
        match val {
            Value::RetVals(rv) => {
                let mut flat_rv = flatten_values(rv);
                flat.append(&mut flat_rv);
            },
            _ => flat.push(val),
        }
    }
    flat
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
            Value::RetVals(rv) => write!(f, "Return values: {rv:?}")
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
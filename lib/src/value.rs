use std::{fmt::Debug, hash::Hash, rc::Rc};

use crate::{ast::{function::Function, Block}, value::table::Table};

pub mod meta;
pub mod table;

#[derive(Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(Boolean),
    Number(f64),
    String(String),
    Userdata,
    Function(Function),
    Thread,
    Table(Rc<Table>),
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
            Value::Table(_) => "Table",
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

// FIXME: SUCKS!
impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
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
                Rc::as_ptr(tb).hash(state);
            },
            Value::RetVals(rv) => {
                state.write_u8(9);
                rv.hash(state);
            },
        }
    }
}

// BAD! VERY VERY BAD!
impl Eq for Value {}

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
            Value::Table(_) => write!(f, "Table"),
            Value::RetVals(rv) => write!(f, "Return values: {rv:?}")
        }?;
        write!(f, " ]")
    }
}

// just putting this here bc its so simple
// FIXME: why is this not just a bool lmao?

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
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
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Nil => { matches!(other, Value::Nil) },
            Value::Boolean(b) => { 
                match other {
                    Value::Boolean(b_other) => { b == b_other }
                    _ => false,
                }
            },
            Value::Number(n) => {
                match other {
                    Value::Number(n_other) => { n == n_other }
                    _ => false,
                }
            }
            _ => panic!("Equality check not implemented for value {}", self.val_str())
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
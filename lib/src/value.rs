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

// just putting this here bc its so simple

#[derive(Clone, Copy)]
pub enum Boolean {
    True, 
    False,
}
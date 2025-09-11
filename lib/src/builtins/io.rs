use std::{cell::RefCell, io, rc::Rc};

use crate::{ast::function::Function, value::{table::Table, Value}};

fn write(args: &Vec<Value>) -> Vec<Value> {
    for arg in args {
        match arg {
            Value::String(s) => print!("{s}"),
            Value::Number(n) => print!("{n}"),
            _ => print!("{:?}", arg)
        }
    }
    Vec::new()
}

fn read(args: &Vec<Value>) -> Vec<Value> {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf).unwrap();

    // check args
    if let Some(Value::String(s)) = args.first() 
    && &s.as_str()[0..1] == "n" 
    {
        vec![Value::Number(buf.trim().parse().unwrap())]
    } else {
        vec![Value::String(buf)]
    }    
}

pub fn create_io_table() -> Rc<RefCell<Table>> {
    let t = Table::new();
    t.borrow_mut().insert(
        &Value::String(String::from("write")),
        Value::Function(Rc::new(Function::Builtin(write))) 
    );

    t.borrow_mut().insert(
        &Value::String(String::from("read")),
        Value::Function(Rc::new(Function::Builtin(read))) 
    );

    t
}
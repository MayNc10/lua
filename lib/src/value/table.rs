use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

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
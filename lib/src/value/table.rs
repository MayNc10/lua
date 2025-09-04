use std::{collections::HashMap, rc::Rc};

use crate::value::Value;

#[derive(Clone)]
pub struct Table {
    inner: HashMap<Value, Value>
}

impl Table {
    pub fn new() -> Rc<Table> {
        Rc::new(Table { inner: HashMap::new() })
    }

    pub fn insert(&mut self, key: &Value, val: Value) {
        if *key == Value::Nil {
            panic!("key shouldnt be null!")
        }

        if val == Value::Nil {
            self.inner.remove(key);
        }
        else {
            // FIXME
            *self.inner.get_mut(key).unwrap() = val;
        }
    }

    pub fn get(&self, key: &Value) -> Value {
        if *key == Value::Nil {
            panic!("key shouldnt be null!")
        }

        self.inner[key].clone()
    }
}
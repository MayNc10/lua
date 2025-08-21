use std::collections::HashMap;

use crate::{lexer::identifier::Identifier, value::Value};


/// Holds current state context
pub struct Ctx {
    // FIXME, SHOULD BE TABLE
    level: usize,
    globals: HashMap<Identifier, Value>,
    locals: HashMap<Identifier, Vec<(Value, usize)>>,
    ret_val: Option<Value>, 
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx { level: 0, globals: HashMap::new(), locals: HashMap::new(), ret_val: None }
    }

    pub fn get_var(&self, ident: &Identifier) -> Option<Value> {
        self.locals
            .get(ident)
            .map_or_else( 
        ||self.globals.get(ident), 
                | val | val.last().map(|(val, _)| val))
            .map(|v| v.clone())
    }

    pub fn new_global(&mut self, ident: Identifier, val: Value) {
        self.globals.insert(ident, val);
    }

    pub fn new_local(&mut self, ident: Identifier, val: Value) {
        self.locals.entry(ident).or_insert(Vec::new()).push((val, self.level));
    }

    pub fn enter_block(&mut self) {
        self.level += 1;
    }

    pub fn leave_block(&mut self) -> Option<Value> {
        self.level -= 1;
        // get rid of old locals
        // there must be a better way to do this
        let mut completely_empty = Vec::new();
        for (ident, values) in self.locals.iter_mut() {
            *values = values.clone().into_iter().filter(|(_, level)| *level < self.level).collect();
            values.sort_by(|(_, l1), (_, l2)| l1.cmp(l2));
            if values.is_empty() {
                completely_empty.push(ident.clone());
            }
        }
        for ident in completely_empty {
            self.locals.remove(&ident);
        }
        self.ret_val.take()
    }
}
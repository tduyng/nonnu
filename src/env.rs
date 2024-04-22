use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    bindings: HashMap<String, Val>,
}

impl Env {
    pub fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }
}

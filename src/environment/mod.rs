use std::collections::HashMap;

pub struct Environment {
    vars: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, key: String, val: String) {
        self.vars.insert(key, val);
    }
}